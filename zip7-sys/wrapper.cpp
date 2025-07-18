#include "wrapper.h"

#include "libzip7/CPP/7zip/Archive/Zip/ZipHandler.h"
#include "libzip7/CPP/7zip/Common/FileStreams.h"
#include "libzip7/CPP/7zip/IPassword.h"

static const UInt32 MAX_ITEMS_COUNT = 10000;

class CArchiveOpenCallback Z7_final : public IArchiveOpenCallback,
                                      public ICryptoGetTextPassword,
                                      public CMyUnknownImp
{
    Z7_IFACES_IMP_UNK_2(IArchiveOpenCallback, ICryptoGetTextPassword)
public:
    UString m_password;
    bool m_askPassword;

    CArchiveOpenCallback() : m_askPassword(false) {}
};

Z7_COM7F_IMF(CArchiveOpenCallback::SetTotal(const UInt64 * /* files */, const UInt64 * /* bytes */))
{
    return S_OK;
}

Z7_COM7F_IMF(CArchiveOpenCallback::SetCompleted(const UInt64 * /* files */, const UInt64 * /* bytes */))
{
    return S_OK;
}

Z7_COM7F_IMF(CArchiveOpenCallback::CryptoGetTextPassword(BSTR *password))
{
    m_askPassword = true;
    return StringToBstr(m_password, password);
}

void init()
{
}

LONG open_archive(const char *path, const char *password, Handle **handle)
{
    auto in_stream_spec = new CInFileStream();
    if (!in_stream_spec->Open(path))
    {
        delete in_stream_spec;
        return E_FAIL;
    }
    CMyComPtr<IInStream> in_stream(in_stream_spec);

    *handle = new Handle();
    if (password)
    {
        (*handle)->password = GetUnicodeString(password);
    }

    NArchive::NZip::CHandler *in_archive_spec = new NArchive::NZip::CHandler();
    (*handle)->in_archive = in_archive_spec;

    UInt64 max_check_start_pos = 1 << 23;
    auto open_callback = new CArchiveOpenCallback;
    open_callback->m_password = (*handle)->password;
    auto res = (*handle)->in_archive->Open(
        in_stream,
        &max_check_start_pos,
        CMyComPtr<IArchiveOpenCallback>(open_callback));
    if (res != S_OK)
    {
        close_archive(*handle);
        if (open_callback->m_askPassword && res == S_FALSE)
        {
            return NArchive::NExtract::NOperationResult::kWrongPassword;
        }
        return res;
    }

    UInt32 items_count = 0;
    (*handle)->in_archive->GetNumberOfItems(&items_count);
    if (items_count > MAX_ITEMS_COUNT)
    {
        items_count = MAX_ITEMS_COUNT;
    }
    (*handle)->items_count = items_count;
    (*handle)->out_paths = new FString[items_count];

    return S_OK;
}

UInt32 items_count(const Handle *handle)
{
    if (!handle || !handle->in_archive)
        return 0;

    UInt32 numItems = 0;
    handle->in_archive->GetNumberOfItems(&numItems);
    return numItems;
}

void close_archive(Handle *handle)
{
    if (handle)
    {
        if (handle->in_archive)
            handle->in_archive->Close();

        delete handle;
        handle = nullptr;
    }
}
