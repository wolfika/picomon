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
    if (live.meta) document.title = `${live.meta.title} · Picomon`;
  });
</script>

<div class="page">
  <header class="masthead">
    <div class="ident">
      <div class="brand">
        <svg viewBox="0 0 32 32" width="20" height="20" aria-hidden="true">
          <rect width="32" height="32" rx="8" fill="var(--accent)" />
          <path d="M5 18h5l3-8 5 14 3-6h6" fill="none" style:stroke="var(--on-accent, #fff)" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
        Picomon
      </div>
      <h1>{live.meta?.title ?? 'Connecting…'}</h1>
      {#if live.meta}
        {@const h = live.meta.host}
        <p class="host dots">
          <span>{h.os}</span>
          <span>Linux {h.kernel}</span>
          {#if h.cpu_model}<span>{h.cpu_model}</span>{/if}
          <span>{h.cpus} threads</span>
          <span>{h.arch}</span>
        </p>
      {/if}
    </div>
    <div class="side">
      {#if live.sample}
        <div class="uptime">
          <span>Uptime</span>
          <b>{duration(live.sample.uptime)}</b>
        </div>
      {/if}
      <div class="controls">
        <span class="conn" class:on={live.connected}>
          <i aria-hidden="true"></i>{live.connected ? 'Live' : 'Reconnecting…'}
        </span>
        <ThemeMenu />
      </div>
    </div>
  </header>

  {#if live.meta && live.sample}
    {@const { meta, sample, history } = live}
    {@const hasTemp = sample.cpu.temp !== null}
    <main class="grid">
      <Cpu {meta} {sample} {history} />
      {#if sample.cpu.temp !== null}
        <Temp {meta} temp={sample.cpu.temp} {history} />
      {/if}
      <Memory {meta} {sample} {history} />
      <!-- Without the temperature card the first row is CPU, memory and storage,
           so the network card takes the whole second row. -->
      <Network {meta} {sample} {history} class={hasTemp ? 'wide' : 'wide full'} />
      <Disks {meta} {sample} {history} />
    </main>
  {:else}
    <p class="empty muted">Waiting for the first sample…</p>
  {/if}
</div>

<style>
  .masthead {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    gap: 20px 32px;
    padding: 24px 28px;
    border-radius: var(--radius);
    background: var(--hero), var(--surface);
    border: 1px solid var(--border);
    box-shadow: var(--shadow);
    backdrop-filter: var(--blur);
    -webkit-backdrop-filter: var(--blur);
  }
  .ident {
    min-width: 0;
    flex: 1 1 420px;
  }
  .brand {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
    font-weight: 700;
    letter-spacing: 0.16em;
    text-transform: uppercase;
    color: var(--muted);
  }
  .brand svg {
    border-radius: 6px;
  }
  h1 {
    margin: 10px 0 6px;
    font-size: 34px;
    font-weight: 750;
    letter-spacing: -0.02em;
    line-height: 1.1;
    overflow-wrap: anywhere;
  }
  .host {
    margin: 0;
    color: var(--muted);
  }
  .side {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    justify-content: space-between;
    gap: 14px;
  }
  .uptime {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    padding: 10px 16px;
    border-radius: 14px;
    border: 1px solid var(--border);
    background: color-mix(in srgb, var(--bg) 35%, transparent);
  }
  .uptime span {
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.14em;
    text-transform: uppercase;
    color: var(--faint);
  }
  .uptime b {
    font-size: 22px;
    font-weight: 700;
    letter-spacing: -0.01em;
  }
  .controls {
    display: flex;
    align-items: center;
    gap: 10px;
  }
  .conn {
    display: inline-flex;
    align-items: center;
    gap: 7px;
    font-size: 13px;
    color: var(--muted);
  }
  .conn i {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--warn);
  }
  .conn.on i {
    background: var(--ok);
    box-shadow: 0 0 0 3px color-mix(in srgb, var(--ok) 22%, transparent);
  }

  /* Narrow screens: uptime and controls sit on one row under the host details. */
  @media (max-width: 640px) {
    .masthead {
      padding: 20px;
    }
    h1 {
      font-size: 28px;
    }
    .side {
      flex: 1 1 100%;
      flex-direction: row;
      align-items: center;
    }
    .uptime {
      align-items: flex-start;
    }
  }
</style>
