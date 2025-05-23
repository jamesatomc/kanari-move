
<a name="0x3_address_mapping"></a>

# Module `0x3::address_mapping`



-  [Resource `MultiChainAddressMapping`](#0x3_address_mapping_MultiChainAddressMapping)
-  [Resource `KanariToBitcoinAddressMapping`](#0x3_address_mapping_KanariToBitcoinAddressMapping)
-  [Constants](#@Constants_0)
-  [Function `genesis_init`](#0x3_address_mapping_genesis_init)
-  [Function `resolve`](#0x3_address_mapping_resolve)
-  [Function `resolve_bitcoin`](#0x3_address_mapping_resolve_bitcoin)
-  [Function `resolve_bitcoin_batch`](#0x3_address_mapping_resolve_bitcoin_batch)
-  [Function `exists_mapping`](#0x3_address_mapping_exists_mapping)
-  [Function `bind_bitcoin_address_internal`](#0x3_address_mapping_bind_bitcoin_address_internal)
-  [Function `bind_bitcoin_address_by_system`](#0x3_address_mapping_bind_bitcoin_address_by_system)
-  [Function `bind_bitcoin_address`](#0x3_address_mapping_bind_bitcoin_address)


<pre><code><b>use</b> <a href="">0x1::option</a>;
<b>use</b> <a href="">0x1::vector</a>;
<b>use</b> <a href="">0x2::core_addresses</a>;
<b>use</b> <a href="">0x2::object</a>;
<b>use</b> <a href="bitcoin_address.md#0x3_bitcoin_address">0x3::bitcoin_address</a>;
<b>use</b> <a href="multichain_address.md#0x3_multichain_address">0x3::multichain_address</a>;
</code></pre>



<a name="0x3_address_mapping_MultiChainAddressMapping"></a>

## Resource `MultiChainAddressMapping`

Mapping from multi-chain address to kanari address
Not including Bitcoin address, because Bitcoin address can directly hash to kanari address
The mapping record is the object field, key is the multi-chain address, value is the kanari address


<pre><code><b>struct</b> <a href="address_mapping.md#0x3_address_mapping_MultiChainAddressMapping">MultiChainAddressMapping</a> <b>has</b> key
</code></pre>



<a name="0x3_address_mapping_KanariToBitcoinAddressMapping"></a>

## Resource `KanariToBitcoinAddressMapping`

Mapping from kanari address to bitcoin address, other chain can use new table
The mapping record is the object field, key is the kanari address, value is the Bitcoin address


<pre><code><b>struct</b> <a href="address_mapping.md#0x3_address_mapping_KanariToBitcoinAddressMapping">KanariToBitcoinAddressMapping</a> <b>has</b> key
</code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="0x3_address_mapping_ErrorMultiChainAddressInvalid"></a>



<pre><code><b>const</b> <a href="address_mapping.md#0x3_address_mapping_ErrorMultiChainAddressInvalid">ErrorMultiChainAddressInvalid</a>: u64 = 1;
</code></pre>



<a name="0x3_address_mapping_ErrorUnsupportedAddress"></a>



<pre><code><b>const</b> <a href="address_mapping.md#0x3_address_mapping_ErrorUnsupportedAddress">ErrorUnsupportedAddress</a>: u64 = 2;
</code></pre>



<a name="0x3_address_mapping_NAMED_MAPPING_INDEX"></a>



<pre><code><b>const</b> <a href="address_mapping.md#0x3_address_mapping_NAMED_MAPPING_INDEX">NAMED_MAPPING_INDEX</a>: u64 = 0;
</code></pre>



<a name="0x3_address_mapping_NAMED_REVERSE_MAPPING_INDEX"></a>



<pre><code><b>const</b> <a href="address_mapping.md#0x3_address_mapping_NAMED_REVERSE_MAPPING_INDEX">NAMED_REVERSE_MAPPING_INDEX</a>: u64 = 1;
</code></pre>



<a name="0x3_address_mapping_genesis_init"></a>

## Function `genesis_init`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="address_mapping.md#0x3_address_mapping_genesis_init">genesis_init</a>(_genesis_account: &<a href="">signer</a>)
</code></pre>



<a name="0x3_address_mapping_resolve"></a>

## Function `resolve`

Resolve a multi-chain address to a kanari address


<pre><code><b>public</b> <b>fun</b> <a href="address_mapping.md#0x3_address_mapping_resolve">resolve</a>(maddress: <a href="multichain_address.md#0x3_multichain_address_MultiChainAddress">multichain_address::MultiChainAddress</a>): <a href="_Option">option::Option</a>&lt;<b>address</b>&gt;
</code></pre>



<a name="0x3_address_mapping_resolve_bitcoin"></a>

## Function `resolve_bitcoin`

Resolve a kanari address to a bitcoin address


<pre><code><b>public</b> <b>fun</b> <a href="address_mapping.md#0x3_address_mapping_resolve_bitcoin">resolve_bitcoin</a>(kanari_address: <b>address</b>): <a href="_Option">option::Option</a>&lt;<a href="bitcoin_address.md#0x3_bitcoin_address_BitcoinAddress">bitcoin_address::BitcoinAddress</a>&gt;
</code></pre>



<a name="0x3_address_mapping_resolve_bitcoin_batch"></a>

## Function `resolve_bitcoin_batch`

Resolve a batch kanari addresses to bitcoin addresses


<pre><code><b>public</b> <b>fun</b> <a href="address_mapping.md#0x3_address_mapping_resolve_bitcoin_batch">resolve_bitcoin_batch</a>(kanari_addresses: <a href="">vector</a>&lt;<b>address</b>&gt;): <a href="">vector</a>&lt;<a href="bitcoin_address.md#0x3_bitcoin_address_BitcoinAddress">bitcoin_address::BitcoinAddress</a>&gt;
</code></pre>



<a name="0x3_address_mapping_exists_mapping"></a>

## Function `exists_mapping`

Check if a multi-chain address is bound to a kanari address


<pre><code><b>public</b> <b>fun</b> <a href="address_mapping.md#0x3_address_mapping_exists_mapping">exists_mapping</a>(maddress: <a href="multichain_address.md#0x3_multichain_address_MultiChainAddress">multichain_address::MultiChainAddress</a>): bool
</code></pre>



<a name="0x3_address_mapping_bind_bitcoin_address_internal"></a>

## Function `bind_bitcoin_address_internal`



<pre><code><b>public</b> <b>fun</b> <a href="address_mapping.md#0x3_address_mapping_bind_bitcoin_address_internal">bind_bitcoin_address_internal</a>(kanari_address: <b>address</b>, btc_address: <a href="bitcoin_address.md#0x3_bitcoin_address_BitcoinAddress">bitcoin_address::BitcoinAddress</a>)
</code></pre>



<a name="0x3_address_mapping_bind_bitcoin_address_by_system"></a>

## Function `bind_bitcoin_address_by_system`



<pre><code><b>public</b> <b>fun</b> <a href="address_mapping.md#0x3_address_mapping_bind_bitcoin_address_by_system">bind_bitcoin_address_by_system</a>(system: &<a href="">signer</a>, kanari_address: <b>address</b>, btc_address: <a href="bitcoin_address.md#0x3_bitcoin_address_BitcoinAddress">bitcoin_address::BitcoinAddress</a>)
</code></pre>



<a name="0x3_address_mapping_bind_bitcoin_address"></a>

## Function `bind_bitcoin_address`

Bind a bitcoin address to a kanari address
We can calculate the kanari address from bitcoin address
So we call this function for record kanari address to bitcoin address mapping


<pre><code><b>public</b> <b>fun</b> <a href="address_mapping.md#0x3_address_mapping_bind_bitcoin_address">bind_bitcoin_address</a>(btc_address: <a href="bitcoin_address.md#0x3_bitcoin_address_BitcoinAddress">bitcoin_address::BitcoinAddress</a>)
</code></pre>
