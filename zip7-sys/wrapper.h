#define INITGUID
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
bool item_is_dir(const Handle *handle, UInt32 index);
UInt64 item_unpacked_size(const Handle *handle, UInt32 index);
unsigned item_path_len(const Handle *handle, UInt32 index);
void item_path(const Handle *handle, UInt32 index, BSTR path);
void set_item_out_path(Handle *handle, UInt32 index, const char *path);
unsigned item_out_path_len(const Handle *handle, UInt32 index);
void item_out_path(const Handle *handle, UInt32 index, char *path);
void close_archive(Handle *handle);
