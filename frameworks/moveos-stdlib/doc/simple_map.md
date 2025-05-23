
<a name="0x2_simple_map"></a>

# Module `0x2::simple_map`

Do some refator because we do not support inline and lambda yet.
This module provides a solution for unsorted maps, that is it has the properties that
1) Keys point to Values
2) Each Key must be unique
3) A Key can be found within O(N) time
4) The keys are unsorted.
5) Adds and removals take O(N) time


-  [Struct `SimpleMap`](#0x2_simple_map_SimpleMap)
-  [Struct `Element`](#0x2_simple_map_Element)
-  [Constants](#@Constants_0)
-  [Function `length`](#0x2_simple_map_length)
-  [Function `new`](#0x2_simple_map_new)
-  [Function `clone`](#0x2_simple_map_clone)
-  [Function `borrow`](#0x2_simple_map_borrow)
-  [Function `borrow_with_default`](#0x2_simple_map_borrow_with_default)
-  [Function `borrow_mut`](#0x2_simple_map_borrow_mut)
-  [Function `contains_key`](#0x2_simple_map_contains_key)
-  [Function `destroy_empty`](#0x2_simple_map_destroy_empty)
-  [Function `add`](#0x2_simple_map_add)
-  [Function `upsert`](#0x2_simple_map_upsert)
-  [Function `keys`](#0x2_simple_map_keys)
-  [Function `values`](#0x2_simple_map_values)
-  [Function `to_vec_pair`](#0x2_simple_map_to_vec_pair)
-  [Function `remove`](#0x2_simple_map_remove)


<pre><code><b>use</b> <a href="">0x1::option</a>;
<b>use</b> <a href="">0x1::vector</a>;
</code></pre>



<a name="0x2_simple_map_SimpleMap"></a>

## Struct `SimpleMap`



<pre><code><b>struct</b> <a href="simple_map.md#0x2_simple_map_SimpleMap">SimpleMap</a>&lt;Key, Value&gt; <b>has</b> <b>copy</b>, drop, store
</code></pre>



<a name="0x2_simple_map_Element"></a>

## Struct `Element`



<pre><code><b>struct</b> <a href="simple_map.md#0x2_simple_map_Element">Element</a>&lt;Key, Value&gt; <b>has</b> <b>copy</b>, drop, store
</code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="0x2_simple_map_ErrorKeyAlreadyExists"></a>

Map key already exists


<pre><code><b>const</b> <a href="simple_map.md#0x2_simple_map_ErrorKeyAlreadyExists">ErrorKeyAlreadyExists</a>: u64 = 1;
</code></pre>



<a name="0x2_simple_map_ErrorKeyNotFound"></a>

Map key is not found


<pre><code><b>const</b> <a href="simple_map.md#0x2_simple_map_ErrorKeyNotFound">ErrorKeyNotFound</a>: u64 = 2;
</code></pre>



<a name="0x2_simple_map_length"></a>

## Function `length`



<pre><code><b>public</b> <b>fun</b> <a href="simple_map.md#0x2_simple_map_length">length</a>&lt;Key, Value&gt;(map: &<a href="simple_map.md#0x2_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;Key, Value&gt;): u64
</code></pre>



<a name="0x2_simple_map_new"></a>

## Function `new`

Create an empty SimpleMap.


<pre><code><b>public</b> <b>fun</b> <a href="simple_map.md#0x2_simple_map_new">new</a>&lt;Key, Value&gt;(): <a href="simple_map.md#0x2_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;Key, Value&gt;
</code></pre>



<a name="0x2_simple_map_clone"></a>

## Function `clone`



<pre><code><b>public</b> <b>fun</b> <a href="simple_map.md#0x2_simple_map_clone">clone</a>&lt;Key: <b>copy</b>, store, Value: <b>copy</b>, store&gt;(map: &<a href="simple_map.md#0x2_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;Key, Value&gt;): <a href="simple_map.md#0x2_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;Key, Value&gt;
</code></pre>



<a name="0x2_simple_map_borrow"></a>

## Function `borrow`



<pre><code><b>public</b> <b>fun</b> <a href="simple_map.md#0x2_simple_map_borrow">borrow</a>&lt;Key, Value&gt;(map: &<a href="simple_map.md#0x2_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;Key, Value&gt;, key: &Key): &Value
</code></pre>



<a name="0x2_simple_map_borrow_with_default"></a>

## Function `borrow_with_default`



<pre><code><b>public</b> <b>fun</b> <a href="simple_map.md#0x2_simple_map_borrow_with_default">borrow_with_default</a>&lt;Key, Value&gt;(map: &<a href="simple_map.md#0x2_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;Key, Value&gt;, key: &Key, default: &Value): &Value
</code></pre>



<a name="0x2_simple_map_borrow_mut"></a>

## Function `borrow_mut`



<pre><code><b>public</b> <b>fun</b> <a href="simple_map.md#0x2_simple_map_borrow_mut">borrow_mut</a>&lt;Key, Value&gt;(map: &<b>mut</b> <a href="simple_map.md#0x2_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;Key, Value&gt;, key: &Key): &<b>mut</b> Value
</code></pre>



<a name="0x2_simple_map_contains_key"></a>

## Function `contains_key`



<pre><code><b>public</b> <b>fun</b> <a href="simple_map.md#0x2_simple_map_contains_key">contains_key</a>&lt;Key, Value&gt;(map: &<a href="simple_map.md#0x2_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;Key, Value&gt;, key: &Key): bool
</code></pre>



<a name="0x2_simple_map_destroy_empty"></a>

## Function `destroy_empty`



<pre><code><b>public</b> <b>fun</b> <a href="simple_map.md#0x2_simple_map_destroy_empty">destroy_empty</a>&lt;Key, Value&gt;(map: <a href="simple_map.md#0x2_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;Key, Value&gt;)
</code></pre>



<a name="0x2_simple_map_add"></a>

## Function `add`



<pre><code><b>public</b> <b>fun</b> <a href="simple_map.md#0x2_simple_map_add">add</a>&lt;Key, Value&gt;(map: &<b>mut</b> <a href="simple_map.md#0x2_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;Key, Value&gt;, key: Key, value: Value)
</code></pre>



<a name="0x2_simple_map_upsert"></a>

## Function `upsert`

Insert key/value pair or update an existing key to a new value


<pre><code><b>public</b> <b>fun</b> <a href="simple_map.md#0x2_simple_map_upsert">upsert</a>&lt;Key, Value&gt;(map: &<b>mut</b> <a href="simple_map.md#0x2_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;Key, Value&gt;, key: Key, value: Value): (<a href="_Option">option::Option</a>&lt;Key&gt;, <a href="_Option">option::Option</a>&lt;Value&gt;)
</code></pre>



<a name="0x2_simple_map_keys"></a>

## Function `keys`

Return all keys in the map. This requires keys to be copyable.


<pre><code><b>public</b> <b>fun</b> <a href="simple_map.md#0x2_simple_map_keys">keys</a>&lt;Key: <b>copy</b>, Value&gt;(map: &<a href="simple_map.md#0x2_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;Key, Value&gt;): <a href="">vector</a>&lt;Key&gt;
</code></pre>



<a name="0x2_simple_map_values"></a>

## Function `values`

Return all values in the map. This requires values to be copyable.


<pre><code><b>public</b> <b>fun</b> <a href="simple_map.md#0x2_simple_map_values">values</a>&lt;Key, Value: <b>copy</b>&gt;(map: &<a href="simple_map.md#0x2_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;Key, Value&gt;): <a href="">vector</a>&lt;Value&gt;
</code></pre>



<a name="0x2_simple_map_to_vec_pair"></a>

## Function `to_vec_pair`

Transform the map into two vectors with the keys and values respectively
Primarily used to destroy a map


<pre><code><b>public</b> <b>fun</b> <a href="simple_map.md#0x2_simple_map_to_vec_pair">to_vec_pair</a>&lt;Key, Value&gt;(map: <a href="simple_map.md#0x2_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;Key, Value&gt;): (<a href="">vector</a>&lt;Key&gt;, <a href="">vector</a>&lt;Value&gt;)
</code></pre>



<a name="0x2_simple_map_remove"></a>

## Function `remove`



<pre><code><b>public</b> <b>fun</b> <a href="simple_map.md#0x2_simple_map_remove">remove</a>&lt;Key, Value&gt;(map: &<b>mut</b> <a href="simple_map.md#0x2_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;Key, Value&gt;, key: &Key): (Key, Value)
</code></pre>
