#include "aurora-rs-mcp/src/cxx/cpp/qnetwork/cxx_qnetwork.h"
#include "aurora-rs-mcp/src/cxx/rust/cxx_qnetwork.rs.h"

namespace com
{
  namespace keygenqt
  {
    namespace aurora_rs_mcp
    {
        std::unique_ptr<CxxQnetwork> new_cxx_qnetwork()
        {
            return std::make_unique<CxxQnetwork>();
        }

        CxxQnetwork::CxxQnetwork() {}

        bool CxxQnetwork::is_online() const
        {
            return m_manager.isOnline();
        }
    } // namespace aurora_rs_mcp
  } // namespace keygenqt
} // namespace com

#include "cxx_qnetwork.moc"
