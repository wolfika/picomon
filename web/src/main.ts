import { mount } from 'svelte';
import App from './App.svelte';
import { live } from './lib/stream.svelte';
import { restoreTheme } from './lib/theme';
import './app.css';

restoreTheme();
live.connect();
mount(App, { target: document.getElementById('app')! });
