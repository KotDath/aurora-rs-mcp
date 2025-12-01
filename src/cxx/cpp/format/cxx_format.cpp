#include "aurora-rs-mcp/src/cxx/cpp/format/cxx_format.h"
#include "aurora-rs-mcp/src/cxx/rust/cxx_format.rs.h"

#include <ctime>
#include <cstring>

namespace ru
{
  namespace kotdath
  {
    namespace aurora_rs_mcp
    {
      std::unique_ptr<CxxFormat> new_format()
      {
        return std::make_unique<CxxFormat>();
      }

      CxxFormat::CxxFormat() {}

      rust::String CxxFormat::time(rust::String format)
      {
        std::time_t time = std::time({});
        char formatData[format.size()];
        strcpy(formatData, format.c_str());
        std::strftime(formatData, format.size(), "%FT%TZ", std::gmtime(&time));
        return formatData;
      }
    } // namespace aurora_rs_mcp
  } // namespace kotdath
} // namespace ru
