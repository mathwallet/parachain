use cumulus_primitives_core::ParaId;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::{ChainType, Properties};
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use mathchain_runtime::{AccountId, Signature};
use sc_telemetry::TelemetryEndpoints;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<mathchain_runtime::GenesisConfig, Extensions>;
const MATHCHAIN_PC1_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

pub fn math_testnet_properties() -> Properties {
	let mut properties = Properties::new();

	properties.insert("ss58Format".into(), 40.into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("tokenSymbol".into(), "MATH".into());

	properties
}

pub fn math_properties() -> Properties {
	let mut properties = Properties::new();

	properties.insert("ss58Format".into(), 39.into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("tokenSymbol".into(), "MATH".into());

	properties
}


/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

pub fn mathchain_pc1_build_spec_config_of(id: ParaId) -> ChainSpec {
	ChainSpec::from_genesis(
		"MathChain PC1",
		"MathChain PC1",
		ChainType::Live,
		move || mathchain_pc1_build_spec_genesis(id),
		vec![],
		Some(
			TelemetryEndpoints::new(vec![(MATHCHAIN_PC1_TELEMETRY_URL.to_string(), 0)])
				.expect("MathChain PC1 telemetry url is valid; qed"),
		),
		// None,
		None,
		Some(math_properties()),
		// None,
		Extensions {
			relay_chain: "rococo".into(),
			para_id: id.into(),
		},
	)
}

fn mathchain_pc1_build_spec_genesis(id: ParaId) -> mathchain_runtime::GenesisConfig {
	const ROOT: &'static str = "0xbc86f5701ad432b63551a677dedfc17ba85d4bce2a608aa2fe26ba246b843909";

	let root = AccountId::from(array_bytes::hex_str_array_unchecked!(ROOT, 32));
	let endowed_accounts = vec![(root.clone(), 1 << 56)];

	mathchain_runtime::GenesisConfig {
		frame_system: mathchain_runtime::SystemConfig {
			code: mathchain_runtime::WASM_BINARY
			.expect("WASM binary was not build, please build it!")
			.to_vec(),
			changes_trie_config: Default::default(),
		},
		pallet_balances: mathchain_runtime::BalancesConfig {
			balances: endowed_accounts,
		},
		pallet_sudo: mathchain_runtime::SudoConfig { key: root },
		parachain_info: mathchain_runtime::ParachainInfoConfig { parachain_id: id },
	}
}

pub fn development_config(id: ParaId) -> ChainSpec {
	ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Local,
		move || {
			testnet_genesis(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				id,
			)
		},
		vec![],
		None,
		None,
		Some(math_properties()),
		Extensions {
			relay_chain: "rococo-dev".into(),
			para_id: id.into(),
		},
	)
}

pub fn local_testnet_config(id: ParaId) -> ChainSpec {
	ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				id,
			)
		},
		vec![],
		None,
		None,
		Some(math_properties()),
		Extensions {
			relay_chain: "rococo-local".into(),
			para_id: id.into(),
		},
	)
}

fn testnet_genesis(
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> mathchain_runtime::GenesisConfig {
	mathchain_runtime::GenesisConfig {
		frame_system: mathchain_runtime::SystemConfig {
			code: mathchain_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			changes_trie_config: Default::default(),
		},
		pallet_balances: mathchain_runtime::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, 1 << 60))
				.collect(),
		},
		pallet_sudo: mathchain_runtime::SudoConfig { key: root_key },
		parachain_info: mathchain_runtime::ParachainInfoConfig { parachain_id: id },
	}
}
