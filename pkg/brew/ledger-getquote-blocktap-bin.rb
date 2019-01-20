class LedgerGetquoteBlocktapBin < Formula
    version '0.0.7'
    desc "Retrieve spot pricing for cryptocurrencies using the Blocktap.io GraphQL API"
    homepage "https://github.com/colindean/ledger-getquote-blocktap"

    if OS.mac?
      url "https://github.com/colindean/ledger-getquote-blocktap/releases/download/#{version}/ledger-getquote-blocktap-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "d8a8b7dd442aa5d12b1838ae355cbb52bc0ecef2d1dd7f31d10982933a67f003"
    elsif OS.linux?
      url "https://github.com/colindean/ledger-getquote-blocktap/releases/download/#{version}/ledger-getquote-blocktap-#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "aff8c32e257ceadcede201ea9e71e65bb905094dabeb0bd6bbb1f6b3277e775b"
    end

    def install
      bin.install "getquote-blocktap"
    end
end