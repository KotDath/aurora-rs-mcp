#pragma once
#include "rust/cxx.h"
#include <memory>

#include <QNetworkConfigurationManager>

namespace ru
{
  namespace kotdath
  {
    namespace aurora_rs_mcp
    {
      class CxxQnetwork : public QObject
      {
        Q_OBJECT

      public:
        CxxQnetwork();
        bool is_online() const;

      private:
        QNetworkConfigurationManager m_manager;
      };

      // Create class
      std::unique_ptr<CxxQnetwork> new_cxx_qnetwork();
    } // namespace aurora_rs_mcp
  } // namespace kotdath
} // namespace ru
