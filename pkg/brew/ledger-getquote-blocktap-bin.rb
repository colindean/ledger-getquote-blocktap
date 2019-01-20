class LedgerGetquoteBlocktapBin < Formula
    version '0.0.5'
    desc "Retrieve spot pricing for cryptocurrencies using the Blocktap.io GraphQL API"
    homepage "https://github.com/colindean/ledger-getquote-blocktap"

    if OS.mac?
      url "https://github.com/colindean/ledger-getquote-blocktap/releases/download/#{version}/ledger-getquote-blocktap-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "afb0f579fa7a447589748b7548b9a718f98940a414ff75185d71bf08ee31e4fb"
    elsif OS.linux?
      url "https://github.com/colindean/ledger-getquote-blocktap/releases/download/#{version}/ledger-getquote-blocktap-#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "2addbe1cf6edc8c9dbd8e9b2329f396683308f528a9a63df0514db6fe9bd9d54"
    end

    def install
      bin.install "getquote-blocktap"
    end
end