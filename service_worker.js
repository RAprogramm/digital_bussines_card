// Динамическое имя кэша с версией
const cacheName = `rozanov-dev-v${new Date().getTime()}`;

// Динамическое кэширование при первой загрузке ресурсов
self.addEventListener('install', function(event) {
  event.waitUntil(
    caches.open(cacheName).then(function(cache) {
      return cache.addAll([
        './',
        './index.html',
        './manifest.json'
      ]);
    })
  );
});

/* Динамическое кэширование запрашиваемых ресурсов */
self.addEventListener('fetch', function(event) {
  event.respondWith(
    caches.match(event.request).then(function(response) {
      if (response) {
        // Запускаем сетевой запрос, чтобы обновить кэш в фоне
        fetch(event.request).then(function(networkResponse) {
          caches.open(cacheName).then(function(cache) {
            cache.put(event.request, networkResponse.clone());
          });
        });
        return response; // возвращаем кэшированный ресурс
      }

      return fetch(event.request).then(function(networkResponse) {
        return caches.open(cacheName).then(function(cache) {
          cache.put(event.request, networkResponse.clone());
          return networkResponse;
        });
      });
    })
  );
});

/* Очистка старого кэша при обновлении Service Worker */
self.addEventListener('activate', function(event) {
  event.waitUntil(
    caches.keys().then(function(cacheNames) {
      return Promise.all(
        cacheNames.map(function(cache) {
          if (cache !== cacheName) {
            return caches.delete(cache);
          }
        })
      );
    })
  );
});
