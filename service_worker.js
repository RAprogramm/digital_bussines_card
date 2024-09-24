var cacheName = 'rozanov-dev-v1';

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
      // Если ресурс уже есть в кэше, возвращаем его
      if (response) {
        return response;
      }

      // Если ресурса нет в кэше, загружаем его и кэшируем
      return fetch(event.request).then(function(networkResponse) {
        return caches.open(cacheName).then(function(cache) {
          // Исключаем из кэширования файлы с запросами через метод POST
          if (event.request.method === 'GET') {
            cache.put(event.request, networkResponse.clone());
          }
          return networkResponse;
        });
      });
    })
  );
});

/* Очистка старого кэша при обновлении Service Worker */
self.addEventListener('activate', function(event) {
  var cacheWhitelist = [cacheName];

  event.waitUntil(
    caches.keys().then(function(cacheNames) {
      return Promise.all(
        cacheNames.map(function(cache) {
          if (cacheWhitelist.indexOf(cache) === -1) {
            return caches.delete(cache);
          }
        })
      );
    })
  );
});
