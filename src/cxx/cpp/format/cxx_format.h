#pragma once
#include "rust/cxx.h"
#include <memory>

namespace com
{
  namespace keygenqt
  {
    namespace aurora_rs_mcp
    {
      class CxxFormat
      {

      public:
        CxxFormat();
        rust::String time(rust::String);
      };

      // Create class
      std::unique_ptr<CxxFormat> new_format();
    } // namespace aurora_rs_mcp
  } // namespace keygenqt
} // namespace com
