<script lang="ts">
  import Cpu from './cards/Cpu.svelte';
  import Disks from './cards/Disks.svelte';
  import Memory from './cards/Memory.svelte';
  import Network from './cards/Network.svelte';
  import Temp from './cards/Temp.svelte';
  import { duration } from './lib/format';
  import { live } from './lib/stream.svelte';
  import ThemeMenu from './lib/ThemeMenu.svelte';

  $effect(() => {
    if (live.meta) document.title = live.meta.title;
  });
</script>

<div class="page">
  <header class="top">
    <div class="ident">
      <h1>{live.meta?.title ?? 'Picomon'}</h1>
      {#if live.meta}
        {@const h = live.meta.host}
        <p class="muted">{h.os} · {h.kernel} · {h.arch}</p>
      {/if}
    </div>
    <div class="status">
      {#if live.sample}
        <span class="muted">Up {duration(live.sample.uptime)}</span>
      {/if}
      <span class="conn" class:on={live.connected}>
        <i aria-hidden="true"></i>{live.connected ? 'Live' : 'Reconnecting…'}
      </span>
      <ThemeMenu />
    </div>
  </header>

  {#if live.meta && live.sample}
    {@const { meta, sample, history } = live}
    <main class="grid">
      <Cpu {meta} {sample} {history} />
      {#if sample.cpu.temp !== null}
        <Temp {meta} temp={sample.cpu.temp} {history} />
      {/if}
      <Memory {meta} {sample} {history} />
      <Network {meta} {sample} {history} />
      <Disks {meta} {sample} {history} />
    </main>
  {:else}
    <p class="empty muted">Waiting for the first sample…</p>
  {/if}
</div>
