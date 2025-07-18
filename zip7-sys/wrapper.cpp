#include "wrapper.h"

#include "libzip7/CPP/7zip/Archive/Zip/ZipHandler.h"
#include "libzip7/CPP/7zip/Common/FileStreams.h"
#include "libzip7/CPP/7zip/Common/RegisterCodec.h"
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

class CArchiveExtractCallback Z7_final : public IArchiveExtractCallback,
                                         public ICryptoGetTextPassword,
                                         public CMyUnknownImp
{
    Z7_IFACES_IMP_UNK_2(IArchiveExtractCallback, ICryptoGetTextPassword)
    Z7_IFACE_COM7_IMP(IProgress)

public:
    UString m_password;
    bool m_encrypted;
    UInt32 m_count;
    UInt32 m_index;
    FString *m_out_paths;

    COutFileStream *m_out_stream_spec;
    CMyComPtr<ISequentialOutStream> m_out_stream;

    CArchiveExtractCallback() : m_encrypted(false),
                                m_out_paths(nullptr),
                                m_out_stream(nullptr)
    {
    }
};

Z7_COM7F_IMF(CArchiveExtractCallback::SetTotal(UInt64 /* size */))
{
    return S_OK;
}

Z7_COM7F_IMF(CArchiveExtractCallback::SetCompleted(const UInt64 * /* completeValue */))
{
    m_out_stream.Release();
    return S_OK;
}

Z7_COM7F_IMF(CArchiveExtractCallback::GetStream(UInt32 index, ISequentialOutStream **outStream, Int32 askExtractMode))
{
    m_out_stream.Release();
    *outStream = nullptr;

    if (askExtractMode != NArchive::NExtract::NAskMode::kExtract)
        return S_OK;

    if (index < m_count && !m_out_paths[index].IsEmpty())
    {
        m_out_stream_spec = new COutFileStream;
        CMyComPtr<IOutStream> out_stream_loc(m_out_stream_spec);
        if (!m_out_stream_spec->Create_ALWAYS(m_out_paths[index]))
        {
            m_out_stream_spec->Close();
            return E_ABORT;
        }

        CMyComPtr<ISequentialOutStream> out_stream(out_stream_loc);
        m_out_stream = out_stream;
        *outStream = m_out_stream.Detach();
    }

    return S_OK;
}

Z7_COM7F_IMF(CArchiveExtractCallback::PrepareOperation(Int32 /* askExtractMode */))
{
    return S_OK;
}

Z7_COM7F_IMF(CArchiveExtractCallback::SetOperationResult(Int32 operationResult))
{
    if (m_out_stream)
    {
        m_out_stream_spec->Close();
    }
    m_out_stream.Release();

    if (m_encrypted && operationResult == NArchive::NExtract::NOperationResult::kDataError)
    {
        return NArchive::NExtract::NOperationResult::kWrongPassword;
    }
    return S_OK;
}

Z7_COM7F_IMF(CArchiveExtractCallback::CryptoGetTextPassword(BSTR *password))
{
    m_encrypted = true;
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

bool item_is_dir(const Handle *handle, UInt32 index)
{
    if (!handle || !handle->in_archive)
        return false;
    NWindows::NCOM::CPropVariant prop;
    handle->in_archive->GetProperty(index, kpidIsDir, &prop);
    if (prop.vt == VT_BOOL)
        return prop.boolVal != VARIANT_FALSE;
    return false;
}

UInt64 item_unpacked_size(const Handle *handle, UInt32 index)
{
    if (!handle || !handle->in_archive)
        return 0;
    NWindows::NCOM::CPropVariant prop;
    handle->in_archive->GetProperty(index, kpidSize, &prop);
    switch (prop.vt)
    {
    case VT_UI8:
        return (UInt64)prop.uhVal.QuadPart;
    case VT_UI4:
        return prop.ulVal;
    case VT_UI2:
        return prop.uiVal;
    case VT_UI1:
        return prop.bVal;
    case VT_EMPTY:
        return 0;
    default:
        return 0;
    }
}

unsigned item_path_len(const Handle *handle, UInt32 index)
{
    if (!handle || !handle->in_archive)
        return 0;
    NWindows::NCOM::CPropVariant prop;
    handle->in_archive->GetProperty(index, kpidPath, &prop);
    if (prop.vt == VT_BSTR)
        return MyStringLen(prop.bstrVal);
    return 0;
}

void item_path(const Handle *handle, UInt32 index, BSTR path)
{
    NWindows::NCOM::CPropVariant prop;
    handle->in_archive->GetProperty(index, kpidPath, &prop);
    if (prop.vt == VT_BSTR)
        MyStringCopy(path, prop.bstrVal);
}

void set_item_out_path(Handle *handle, UInt32 index, const char *path)
{
    if (!handle || index >= handle->items_count)
        return;

    handle->out_paths[index] = path;
}

unsigned item_out_path_len(const Handle *handle, UInt32 index)
{
    if (!handle || index >= handle->items_count)
        return 0;

    return handle->out_paths[index].Len();
}

void item_out_path(const Handle *handle, UInt32 index, char *path)
{
    if (!handle || index >= handle->items_count)
        return;

    MyStringCopy(path, handle->out_paths[index]);
}

LONG extract(const Handle *handle)
{
    if (!handle || !handle->in_archive)
        return E_INVALIDARG;
    auto extract_callback = new CArchiveExtractCallback();
    extract_callback->m_password = handle->password;
    extract_callback->m_count = handle->items_count;
    extract_callback->m_out_paths = handle->out_paths;
    extract_callback->m_out_stream = nullptr;

    return handle->in_archive->Extract(
        nullptr,
        0xFFFFFFFF,
        false,
        CMyComPtr<IArchiveExtractCallback>(extract_callback));
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

// Stubs for linking
namespace NArchive
{
    namespace NZip
    {

        Z7_COM7F_IMF(CHandler::UpdateItems(ISequentialOutStream *, UInt32, IArchiveUpdateCallback *))
        {
            return E_NOTIMPL;
        }

        Z7_COM7F_IMF(CHandler::SetProperties(const wchar_t *const *, const PROPVARIANT *, UInt32))
        {
            return E_NOTIMPL;
        }

        Z7_COM7F_IMF(CHandler::GetFileTimeType(UInt32 *))
        {
            return E_NOTIMPL;
        }
    }
}

extern "C" SizeT z7_BranchConvSt_X86_Dec(Byte *, SizeT, UInt32, UInt32)
{
    return 0;
}
