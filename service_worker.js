var cacheName = 'rozanov-dev';
var filesToCache = [
  './',
  './index.html',
  './manifest.json',
  './fonts/',
  './images/',
  './business-card-19a0b2818c80bf44.js',
  './business-card-19a0b2818c80bf44_bg.wasm',
  './card-967bf27f107bc36d.css',
  './common-72e6c7e34c655ccd.css',
  './english-1ca621f4ecf4e5e.css',
  './korean-3a4be8182e2d81a6.css',
  './russian-ca2210277db8d5ca.css',
  './photo-4c57af31f4f3e898.css',
  './social-6b8195cf07663bd0.css',
  './service_worker.js'
];

/* Start the service worker and cache all of the app's content */
self.addEventListener('install', function(e) {
  e.waitUntil(
    caches.open(cacheName).then(function(cache) {
      return cache.addAll(filesToCache);
    })
  );
});

/* Serve cached content when offline */
self.addEventListener('fetch', function(e) {
  e.respondWith(
    caches.match(e.request).then(function(response) {
      return response || fetch(e.request);
    })
  );
});
