var cacheName = 'rozanov-dev';
var filesToCache = [
  './',
  './index.html',
  './manifest.json',
  './business-card-a2f87776afbff675df192f5ef0b8c15f04dd34cdadbb95453ffc499a0b30ccc256aca3e537d9ae023f93c508b37da209.js',
  './business-card-a2f87776afbff675df192f5ef0b8c15f04dd34cdadbb95453ffc499a0b30ccc256aca3e537d9ae023f93c508b37da209_bg.wasm',
  './card-b51b57eb686c50e4.css',
  './common-d9100199d73e14d.css',
  './english-da5bbfaaf645a9cc.css',
  './korean-8096dc6a98b61425.css',
  './russian-75ade782d1a8cd34.css',
  './photo-279ef7f03220109b.css',
  './social-86957b08a03d9950.css',
  './fonts/',
  './images/'
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
