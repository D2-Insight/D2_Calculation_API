type Hash = number;
/**
*/
export function start(): void;
/**
* @returns {MetaData}
*/
export function getMetadata(): MetaData;
/**
* @returns {string}
*/
export function stringifyWeapon(): string;
/**
*Returns the weapon as a JSON structure, snake case fields
* @returns {any}
*/
export function weaponJSON(): any;
/**
* @param {Hash} _hash
* @param {number} _weapon_type_id
* @param {Hash} _intrinsic_hash
* @param {number} _ammo_type_id
* @param {number} _damage_type_id
*/
export function setWeapon(_hash: Hash, _weapon_type_id: number, _intrinsic_hash: Hash, _ammo_type_id: number, _damage_type_id: number): void;
/**
* @returns {Map<Hash, Stat>}
*/
export function getStats(): Map<Hash, Stat>;
/**
* @param {Map<Hash, number>} _stats
*/
export function setStats(_stats: Map<Hash, number>): void;
/**
* @param {Map<Hash, number>} _stats
* @param {number} _value
* @param {number} _hash
*/
export function addTrait(_stats: Map<Hash, number>, _value: number, _hash: number): void;
/**
* @returns {Array<Hash>}
*/
export function getTraitHashes(): Array<Hash>;
/**
* @param {number} perk_hash
* @param {number} new_value
*/
export function setTraitValue(perk_hash: Hash, new_value: number): void;
/**
* @param {Uint32Array} _perks
* @returns {Map<Hash, PerkOptionData>}
*/
export function getTraitOptions(_perks: Uint32Array): Map<Hash, PerkOptionData>;
/**
* @param {boolean} _dynamic_traits
* @param {boolean} _pvp
* @returns {RangeResponse}
*/
export function getWeaponRangeFalloff(_dynamic_traits: boolean, _pvp: boolean): RangeResponse;
/**
* @param {boolean} _dynamic_traits
* @param {boolean} _pvp
* @returns {HandlingResponse}
*/
export function getWeaponHandlingTimes(_dynamic_traits: boolean, _pvp: boolean): HandlingResponse;
/**
* @param {boolean} _dynamic_traits
* @param {boolean} _pvp
* @returns {ReloadResponse}
*/
export function getWeaponReloadTimes(_dynamic_traits: boolean, _pvp: boolean): ReloadResponse;
/**
* @param {boolean} _dynamic_traits
* @param {boolean} _pvp
* @returns {AmmoResponse}
*/
export function getWeaponAmmoSizes(_dynamic_traits: boolean, _pvp: boolean): AmmoResponse;
/**
* @param {number} _overhsield
* @returns {TtkResponse}
*/
export function getWeaponTtk(_overhsield: number): Array<ResillienceTtkSummary>;
/**
* @param {boolean} _use_rpl
* @returns {DpsResponse}
*/
export function getWeaponDps(_use_rpl: boolean): DpsResponse;
/**
* @param {boolean} _dynamic_traits
* @param {boolean} _pvp
* @param {boolean} _use_rpl
* @returns {FiringResponse}
*/
export function getWeaponFiringData(_dynamic_traits: boolean, _pvp: boolean, _use_rpl: boolean): FiringResponse;
/**
* @param {number} _rpl
* @param {number} _override_cap
* @param {number} _difficulty
* @param {number} _enemy_type
*/
export function setEncounter(_rpl: number, _override_cap: number, _difficulty: number, _enemy_type: number): void;
/**
*/
export enum DifficultyOptions {
  NORMAL,
  RAID,
  MASTER,
}
/**
*/
export enum EnemyType {
  MINOR,
  ELITE,
  MINIBOSS,
  BOSS,
  VEHICLE,
  ENCLAVE,
  PLAYER,
  CHAMPION,
}
/**
*/
export class AmmoResponse {
  /**
  ** Return copy of self without private attributes.
  */
  toJSON(): Object;
  /**
  * Return stringified version of self.
  */
  toString(): string;
  free(): void;
  /**
  */
  readonly magSize: number;
  /**
  */
  readonly reserveSize: number;
}
/**
*/
export class PerkOptionData {
  /**
  */
  stacks: Array<number>;
  /**
  */
  options: Array<string>;
  /**
  */
  optionType: string;
}
/**
*/
export class OptimalKillData {
  /**
  ** Return copy of self without private attributes.
  */
  toJSON(): Object;
  /**
  * Return stringified version of self.
  */
  toString(): string;
  free(): void;
  /**
  */
  readonly achievableRange: number;
  /**
  */
  readonly bodyshots: number;
  /**
  */
  readonly headshots: number;
  /**
  */
  readonly timeTaken: number;
}
/**
*/
export class BodyKillData {
  /**
  ** Return copy of self without private attributes.
  */
  toJSON(): Object;
  /**
  * Return stringified version of self.
  */
  toString(): string;
  free(): void;
  /**
  */
  readonly bodyshots: number;
  /**
  */
  readonly timeTaken: number;
}
/**
*/
export class ResillienceTtkSummary {
  /**
  ** Return copy of self without private attributes.
  */
  toJSON(): Object;
  /**
  * Return stringified version of self.
  */
  toString(): string;
  free(): void;
  /**
  */
  readonly bodyTtk: BodyKillData;
  /**
  */
  readonly optimalTtk: OptimalKillData;
  /**
  */
  readonly resillienceValue: number;
}
/**
*/
export class DpsResponse {
  free(): void;
  /**
  * Return stringified version of self.
  */
  toString(): string;
  /**
  ** Return copy of self without private attributes.
  */
  toJSON(): string;
  /**
  *Returns a list of dps values for each magazine
  */
  readonly dpsPerMag: Array<number>;
  /**
  *Returns a list of tuples of time and damage
  */
  readonly timeDamageData: Array<Array<number>>;
  /**
  */
  readonly totalDamage: number;
  /**
  */
  readonly totalShots: number;
  /**
  */
  readonly totalTime: number;
}
/**
*/
export class FiringResponse {
  /**
  ** Return copy of self without private attributes.
  */
  toJSON(): Object;
  /**
  * Return stringified version of self.
  */
  toString(): string;
  free(): void;
  /**
  */
  readonly burstDelay: number;
  /**
  */
  readonly burstSize: number;
  /**
  */
  readonly innerBurstDelay: number;
  /**
  */
  readonly pveCritMult: number;
  /**
  */
  readonly pveExplosionDamage: number;
  /**
  */
  readonly pveImpactDamage: number;
  /**
  */
  readonly pvpCritMult: number;
  /**
  */
  readonly pvpExplosionDamage: number;
  /**
  */
  readonly pvpImpactDamage: number;
  /**
  */
  readonly rpm: number;
}
/**
*/
export class HandlingResponse {
  /**
  ** Return copy of self without private attributes.
  */
  toJSON(): Object;
  /**
  * Return stringified version of self.
  */
  toString(): string;
  free(): void;
  /**
  */
  readonly adsTime: number;
  /**
  */
  readonly readyTime: number;
  /**
  */
  readonly stowTime: number;
}
/**
*/
export class MetaData {
  /**
  ** Return copy of self without private attributes.
  */
  toJSON(): Object;
  /**
  * Return stringified version of self.
  */
  toString(): string;
  free(): void;
  /**
  */
  readonly apiGitBranch: string;
  /**
  */
  readonly apiGitCommit: string;
  /**
  */
  readonly apiTimestamp: string;
  /**
  */
  readonly apiVersion: string;
  /**
  */
  readonly databaseTimestamp: bigint;
}
/**
*/
export class RangeResponse {
  /**
  ** Return copy of self without private attributes.
  */
  toJSON(): Object;
  /**
  * Return stringified version of self.
  */
  toString(): string;
  free(): void;
  /**
  */
  readonly adsFalloffEnd: number;
  /**
  */
  readonly adsFalloffStart: number;
  /**
  */
  readonly floorPercent: number;
  /**
  */
  readonly hipFalloffEnd: number;
  /**
  */
  readonly hipFalloffStart: number;
}
/**
*/
export class ReloadResponse {
  /**
  ** Return copy of self without private attributes.
  */
  toJSON(): Object;
  /**
  * Return stringified version of self.
  */
  toString(): string;
  free(): void;
  /**
  */
  readonly ammoTime: number;
  /**
  */
  readonly reloadTime: number;
}
/**
*/
export class Stat {
  free(): void;
  /**
  * @returns {string}
  */
  toString(): string;
  /**
  */
  readonly baseValue: number;
  /**
  */
  readonly partValue: number;
  /**
  */
  readonly traitValue: number;
}
/**
*/
export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_handlingresponse_free: (a: number) => void;
  readonly __wbg_reloadresponse_free: (a: number) => void;
  readonly __wbg_ammoresponse_free: (a: number) => void;
  readonly __wbg_get_ammoresponse_magSize: (a: number) => number;
  readonly __wbg_get_ammoresponse_reserveSize: (a: number) => number;
  readonly __wbg_dpsresponse_free: (a: number) => void;
  readonly __wbg_get_dpsresponse_totalTime: (a: number) => number;
  readonly __wbg_get_dpsresponse_totalShots: (a: number) => number;
  readonly dpsresponse_toString: (a: number, b: number) => void;
  readonly dpsresponse_toJSON: (a: number, b: number) => void;
  readonly dpsresponse_timeDamageData: (a: number) => number;
  readonly dpsresponse_dpsPerMag: (a: number) => number;
  readonly __wbg_optimalkilldata_free: (a: number) => void;
  readonly __wbg_get_optimalkilldata_headshots: (a: number) => number;
  readonly __wbg_set_optimalkilldata_headshots: (a: number, b: number) => void;
  readonly __wbg_get_optimalkilldata_bodyshots: (a: number) => number;
  readonly __wbg_set_optimalkilldata_bodyshots: (a: number, b: number) => void;
  readonly __wbg_set_optimalkilldata_achievableRange: (a: number, b: number) => void;
  readonly __wbg_bodykilldata_free: (a: number) => void;
  readonly __wbg_get_bodykilldata_bodyshots: (a: number) => number;
  readonly __wbg_set_bodykilldata_bodyshots: (a: number, b: number) => void;
  readonly __wbg_get_bodykilldata_timeTaken: (a: number) => number;
  readonly __wbg_set_bodykilldata_timeTaken: (a: number, b: number) => void;
  readonly __wbg_resilliencesummary_free: (a: number) => void;
  readonly __wbg_set_resilliencesummary_resillienceValue: (a: number, b: number) => void;
  readonly __wbg_get_resilliencesummary_bodyTtk: (a: number) => number;
  readonly __wbg_set_resilliencesummary_bodyTtk: (a: number, b: number) => void;
  readonly __wbg_get_resilliencesummary_optimalTtk: (a: number) => number;
  readonly __wbg_set_resilliencesummary_optimalTtk: (a: number, b: number) => void;
  readonly __wbg_firingresponse_free: (a: number) => void;
  readonly __wbg_get_firingresponse_pvpCritMult: (a: number) => number;
  readonly __wbg_get_firingresponse_pveImpactDamage: (a: number) => number;
  readonly __wbg_get_firingresponse_pveExplosionDamage: (a: number) => number;
  readonly __wbg_get_firingresponse_pveCritMult: (a: number) => number;
  readonly __wbg_get_firingresponse_burstDelay: (a: number) => number;
  readonly __wbg_get_firingresponse_innerBurstDelay: (a: number) => number;
  readonly __wbg_get_firingresponse_burstSize: (a: number) => number;
  readonly __wbg_get_firingresponse_rpm: (a: number) => number;
  readonly __wbg_set_firingresponse_rpm: (a: number, b: number) => void;
  readonly __wbg_stat_free: (a: number) => void;
  readonly __wbg_set_stat_baseValue: (a: number, b: number) => void;
  readonly __wbg_set_stat_partValue: (a: number, b: number) => void;
  readonly __wbg_get_stat_traitValue: (a: number) => number;
  readonly __wbg_set_stat_traitValue: (a: number, b: number) => void;
  readonly stat_toString: (a: number, b: number) => void;
  readonly __wbg_metadata_free: (a: number) => void;
  readonly __wbg_get_metadata_databaseTimestamp: (a: number) => number;
  readonly __wbg_get_metadata_apiVersion: (a: number, b: number) => void;
  readonly __wbg_get_metadata_apiTimestamp: (a: number, b: number) => void;
  readonly __wbg_get_metadata_apiGitCommit: (a: number, b: number) => void;
  readonly __wbg_get_metadata_apiGitBranch: (a: number, b: number) => void;
  readonly start: () => void;
  readonly getMetadata: (a: number) => void;
  readonly stringifyWeapon: (a: number) => void;
  readonly weaponJSON: (a: number) => void;
  readonly setWeapon: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly getStats: (a: number) => void;
  readonly setStats: (a: number, b: number) => void;
  readonly addTrait: (a: number, b: number, c: number, d: number) => void;
  readonly getTraitHashes: (a: number) => void;
  readonly setTraitValue: (a: number, b: number) => void;
  readonly getTraitOptions: (a: number, b: number, c: number) => void;
  readonly getWeaponRangeFalloff: (a: number, b: number) => void;
  readonly getWeaponHandlingTimes: (a: number, b: number) => void;
  readonly getWeaponReloadTimes: (a: number, b: number) => void;
  readonly getWeaponAmmoSizes: (a: number, b: number) => void;
  readonly getWeaponTtk: (a: number, b: number) => void;
  readonly getWeaponDps: (a: number, b: number) => void;
  readonly getWeaponFiringData: (a: number, b: number, c: number) => void;
  readonly setEncounter: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly __wbg_rangeresponse_free: (a: number) => void;
  readonly __wbg_get_rangeresponse_hipFalloffStart: (a: number) => number;
  readonly __wbg_get_reloadresponse_reloadTime: (a: number) => number;
  readonly __wbg_get_handlingresponse_readyTime: (a: number) => number;
  readonly __wbg_get_optimalkilldata_timeTaken: (a: number) => number;
  readonly __wbg_get_dpsresponse_totalDamage: (a: number) => number;
  readonly __wbg_get_firingresponse_pvpImpactDamage: (a: number) => number;
  readonly __wbg_get_stat_baseValue: (a: number) => number;
  readonly __wbg_set_optimalkilldata_timeTaken: (a: number, b: number) => void;
  readonly __wbg_get_rangeresponse_hipFalloffEnd: (a: number) => number;
  readonly __wbg_get_rangeresponse_adsFalloffStart: (a: number) => number;
  readonly __wbg_get_reloadresponse_ammoTime: (a: number) => number;
  readonly __wbg_get_handlingresponse_stowTime: (a: number) => number;
  readonly __wbg_get_optimalkilldata_achievableRange: (a: number) => number;
  readonly __wbg_get_resilliencesummary_resillienceValue: (a: number) => number;
  readonly __wbg_get_firingresponse_pvpExplosionDamage: (a: number) => number;
  readonly __wbg_get_handlingresponse_adsTime: (a: number) => number;
  readonly __wbg_get_rangeresponse_adsFalloffEnd: (a: number) => number;
  readonly __wbg_get_rangeresponse_floorPercent: (a: number) => number;
  readonly __wbg_get_stat_partValue: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init(module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;