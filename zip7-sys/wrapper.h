#include "libzip7/CPP/Common/MyCom.h"
#include "libzip7/CPP/Common/StringConvert.h"
#include "libzip7/CPP/7zip/Archive/IArchive.h"

struct Handle
{
    CMyComPtr<IInArchive> in_archive;
    UString password;
    UInt32 items_count;
    FString *out_paths;
};

void init();
LONG open_archive(const char *path, const char *password, Handle **handle);
UInt32 items_count(const Handle *handle);
void close_archive(Handle *handle);
