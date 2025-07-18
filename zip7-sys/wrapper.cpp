#include "wrapper.h"

#include "libzip7/CPP/7zip/Compress/ShrinkDecoder.h"
#include "libzip7/CPP/7zip/ICoder.h"
#include "libzip7/CPP/Common/MyCom.h"

static CMyComPtr<ICompressCoder> g_shrink_decoder;

void init()
{
    g_shrink_decoder->Release();
    g_shrink_decoder = new NCompress::NShrink::CDecoder;
}

LONG shrink_decode(UInt64 in_size, UInt64 out_size)
{
    CMyComPtr<ISequentialInStream> in_stream;
    CMyComPtr<ISequentialOutStream> out_stream;
    return g_shrink_decoder->Code(in_stream, out_stream, &in_size, &out_size, nullptr);
}
