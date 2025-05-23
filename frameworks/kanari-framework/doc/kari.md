
<a name="0x3_kari"></a>

# Module `0x3::kari`

This module defines Kanari Gas Coin.


-  [Resource `KARI`](#0x3_kari_KARI)
-  [Constants](#@Constants_0)
-  [Function `decimals`](#0x3_kari_decimals)
-  [Function `balance`](#0x3_kari_balance)
-  [Function `burn`](#0x3_kari_burn)
-  [Function `deduct_gas`](#0x3_kari_deduct_gas)
-  [Function `faucet`](#0x3_kari_faucet)
-  [Function `faucet_entry`](#0x3_kari_faucet_entry)
-  [Function `genesis_init`](#0x3_kari_genesis_init)


<pre><code><b>use</b> <a href="">0x1::option</a>;
<b>use</b> <a href="">0x1::string</a>;
<b>use</b> <a href="">0x2::object</a>;
<b>use</b> <a href="">0x2::signer</a>;
<b>use</b> <a href="account_coin_store.md#0x3_account_coin_store">0x3::account_coin_store</a>;
<b>use</b> <a href="chain_id.md#0x3_chain_id">0x3::chain_id</a>;
<b>use</b> <a href="coin.md#0x3_coin">0x3::coin</a>;
<b>use</b> <a href="onchain_config.md#0x3_onchain_config">0x3::onchain_config</a>;
</code></pre>



<a name="0x3_kari_KARI"></a>

## Resource `KARI`

KARI is the symbol of kanari Gas Coin


<pre><code><b>struct</b> <a href="kari.md#0x3_kari_KARI">KARI</a> <b>has</b> store, key
</code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="0x3_kari_DECIMALS"></a>



<pre><code><b>const</b> <a href="kari.md#0x3_kari_DECIMALS">DECIMALS</a>: u8 = 8;
</code></pre>



<a name="0x3_kari_decimals"></a>

## Function `decimals`



<pre><code><b>public</b> <b>fun</b> <a href="kari.md#0x3_kari_decimals">decimals</a>(): u8
</code></pre>



<a name="0x3_kari_balance"></a>

## Function `balance`



<pre><code><b>public</b> <b>fun</b> <a href="kari.md#0x3_kari_balance">balance</a>(addr: <b>address</b>): <a href="">u256</a>
</code></pre>



<a name="0x3_kari_burn"></a>

## Function `burn`



<pre><code><b>public</b> <b>fun</b> <a href="kari.md#0x3_kari_burn">burn</a>(<a href="coin.md#0x3_coin">coin</a>: <a href="coin.md#0x3_coin_Coin">coin::Coin</a>&lt;<a href="kari.md#0x3_kari_KARI">kari::KARI</a>&gt;)
</code></pre>



<a name="0x3_kari_deduct_gas"></a>

## Function `deduct_gas`

deduct gas coin from the given account.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="kari.md#0x3_kari_deduct_gas">deduct_gas</a>(addr: <b>address</b>, amount: <a href="">u256</a>): <a href="coin.md#0x3_coin_Coin">coin::Coin</a>&lt;<a href="kari.md#0x3_kari_KARI">kari::KARI</a>&gt;
</code></pre>



<a name="0x3_kari_faucet"></a>

## Function `faucet`

Mint gas coin to the given account.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="kari.md#0x3_kari_faucet">faucet</a>(addr: <b>address</b>, amount: <a href="">u256</a>)
</code></pre>



<a name="0x3_kari_faucet_entry"></a>

## Function `faucet_entry`

Entry point for the faucet, anyone can get Gas via this function on local/dev net, otherwise only admin account can call this function.


<pre><code><b>public</b> entry <b>fun</b> <a href="kari.md#0x3_kari_faucet_entry">faucet_entry</a>(<a href="">account</a>: &<a href="">signer</a>, amount: <a href="">u256</a>)
</code></pre>



<a name="0x3_kari_genesis_init"></a>

## Function `genesis_init`

Can only be called during genesis to initialize the kanari coin.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="kari.md#0x3_kari_genesis_init">genesis_init</a>(_genesis_account: &<a href="">signer</a>)
</code></pre>
