import { mount } from 'svelte';
import App from './App.svelte';
import { live } from './lib/stream.svelte';
import { restoreTheme } from './lib/theme';
import './app.css';

restoreTheme();
live.connect();
const target = document.getElementById('app');
if (!target) throw new Error('index.html has no #app element');
mount(App, { target });
