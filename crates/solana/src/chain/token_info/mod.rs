// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::Uri;

pub mod local;
pub mod rpc;
pub mod uri;

pub fn sanitize_value(value: impl Into<String>) -> String {
    value.into().replace("\0", "").trim().to_string()
}

// fix cloudflare cf-ipfs urls because the cloudflare gateway service has been discontinued
// affected https://cf-ipfs.com/ipfs/QmSqKqHSbJxomL22qqHnRJiQhyUVmVKP1xDAmasXmqT4k4
// affected https://bafybeifg74lu23rmp7zojbcsxqjpsyt2ico42xduqw35c45f6fhfexwnla.ipfs.cf-ipfs.com/
// good https://bafkreifikbwp45kdcz324uzz4h5ukkziumeqs4dafpdfyn2mulb4xezfte.ipfs.nftstorage.link/
// good https://gateway.pinata.cloud/ipfs/QmW1DTwhBGvwDehUXtMqo1aw61W2i39bAdD6oHhVouzJJ2
// good https://quicknode.quicknode-ipfs.com/ipfs/QmTLBbmgKxHVs1kF4Ze4nFDtd94d66oeaKZmkQPiLAK9sk
pub(crate) fn rewrite_ipfs(uri: impl Into<Uri>) -> Uri {
    // see https://blog.cloudflare.com/cloudflares-public-ipfs-gateways-and-supporting-interplanetary-shipyard/
    // see https://docs.ipfs.tech/how-to/address-ipfs-on-web/#path-gateway
    // note: .replace strings are not regex!
    uri.into()
        .0
        // IPFS HTTP gateways
        .replace("https://cf-ipfs.com/ipfs/", "https://ipfs.io/ipfs/")
        .replace("https://cf-ipfs.com/ipfs", "https://ipfs.io/ipfs")
        .replace("https://cloudflare-ipfs.com/ipfs", "https://ipfs.io/ipfs")
        // IPFS "Subdomain gateway"
        .replace(".ipfs.cf-ipfs.com/", ".ipfs.dweb.link/")
        .replace(".ipfs.cf-ipfs.com", ".ipfs.dweb.link")
        // Pinata IPFS gateway causes rate limits
        .replace(
            "https://gateway.pinata.cloud/ipfs/",
            "https://ipfs.io/ipfs/",
        )
        .into()
}

#[cfg(test)]
mod tests {
    use crate::token_info::{rewrite_ipfs, sanitize_value};

    #[test]
    fn test_ok() {
        assert_eq!(
            rewrite_ipfs("https://cf-ipfs.com/ipfs/QmSqKqHSbJxomL22qqHnRJiQhyUVmVKP1xDAmasXmqT4k4"),
            "https://ipfs.io/ipfs/QmSqKqHSbJxomL22qqHnRJiQhyUVmVKP1xDAmasXmqT4k4"
        );
    }

    #[test]
    fn test_nothing_to_fix() {
        let url = "https://ipfs.io/ipfs/QmSqKqHSbJxomL22qqHnRJiQhyUVmVKP1xDAmasXmqT4k4";
        assert_eq!(
            rewrite_ipfs(url),
            "https://ipfs.io/ipfs/QmSqKqHSbJxomL22qqHnRJiQhyUVmVKP1xDAmasXmqT4k4"
        );
    }

    #[test]
    fn test_fix_url_cf_gateway() {
        assert_eq!(
            rewrite_ipfs("https://cf-ipfs.com/ipfs/QmSqKqHSbJxomL22qqHnRJiQhyUVmVKP1xDAmasXmqT4k4"),
            "https://ipfs.io/ipfs/QmSqKqHSbJxomL22qqHnRJiQhyUVmVKP1xDAmasXmqT4k4"
        );
    }

    #[test]
    fn test_fix_url_cloudflare_gateway() {
        assert_eq!(
            rewrite_ipfs(
                "https://cloudflare-ipfs.com/ipfs/QmVas1L6krA5WiDd2e5QGVfVjNdkMiXv2SXRTP7SoKnfzL"
            ),
            "https://ipfs.io/ipfs/QmVas1L6krA5WiDd2e5QGVfVjNdkMiXv2SXRTP7SoKnfzL"
        );
    }

    #[test]
    fn test_fix_url_subdomain() {
        assert_eq!(
			rewrite_ipfs(
				"https://bafybeifg74lu23rmp7zojbcsxqjpsyt2ico42xduqw35c45f6fhfexwnla.ipfs.cf-ipfs.com"
			),
			"https://bafybeifg74lu23rmp7zojbcsxqjpsyt2ico42xduqw35c45f6fhfexwnla.ipfs.dweb.link"
		);

        assert_eq!(
			rewrite_ipfs(
				"https://bafybeifg74lu23rmp7zojbcsxqjpsyt2ico42xduqw35c45f6fhfexwnla.ipfs.cf-ipfs.com/"
			),
			"https://bafybeifg74lu23rmp7zojbcsxqjpsyt2ico42xduqw35c45f6fhfexwnla.ipfs.dweb.link/"
		);
    }

    #[test]
    fn test_fix_url_pinata() {
        assert_eq!(
            rewrite_ipfs(
                "https://gateway.pinata.cloud/ipfs/QmWnVbXUZNyswneZXX5oNpksr7JXpmYmjogAmBrKTqDQAx"
            ),
            "https://ipfs.io/ipfs/QmWnVbXUZNyswneZXX5oNpksr7JXpmYmjogAmBrKTqDQAx"
        );
    }

    #[test]
    fn sanitize_value_success() {
        assert_eq!(
            sanitize_value(" BOMBO \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            "BOMBO"
        )
    }
}
