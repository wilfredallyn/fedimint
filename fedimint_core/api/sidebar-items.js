window.SIDEBAR_ITEMS = {"constant":[["BECH32_HRP","We can represent client invite code as a bech32 string for compactness and error-checking"],["CONFIG_DOWNLOAD_TOKEN_BYTES","Size of a download token"]],"enum":[["OutputOutcomeError",""],["PeerConnectionStatus",""],["PeerError","An API request error when calling a single federation peer"],["ServerStatus","The state of the server returned via APIs"]],"fn":[["map_tx_outcome_outpoint",""],["url_to_string_with_default_port","`jsonrpsee` converts the `Url` to a `&str` internally and then parses it as an `Uri`. Unfortunately `Url` swallows ports that it considers default ports (e.g. 80 and 443 for HTTP(S)) which makes the `Uri` parsing fail in these cases. This function works around this limitation in a limited way (not fully standard compliant, but work for our use case)."]],"struct":[["ApiVersionSet","Set of api versions for each component (core + modules)"],["ClientConfigDownloadToken","Allows a client to download the config"],["DynGlobalApi",""],["DynModuleApi",""],["FederationError","An API request error when calling an entire federation"],["FederationPeer",""],["FederationStatus","The status of a server, including how it views its peers"],["InviteCode","Information required for client to construct [`WsFederationApi`] instance"],["PeerResponse",""],["PeerStatus",""],["StatusResponse",""],["WsFederationApi","Mint API client that will try to run queries against all `peers` expecting equal results from at least `min_eq_results` of them. Peers that return differing results are returned as a peer faults list."]],"trait":[["FederationApiExt","An extension trait allowing to making federation-wide API call on top [`IFederationApi`]."],["GlobalFederationApi","The API for the global (non-module) endpoints"],["IFederationApi","An API (module or global) that can query a federation"],["IGlobalFederationApi","Trait marker for the global (non-module) endpoints"],["IModuleFederationApi","Trait marker for the module (non-global) endpoints"],["JsonRpcClient",""]],"type":[["FederationResult",""],["JsonRpcResult",""],["OutputOutcomeResult",""],["PeerResult",""]]};