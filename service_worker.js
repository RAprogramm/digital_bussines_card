// Версия кэша
const cacheName = `rozanov-dev-v${new Date().getTime()}`;

// Устанавливаем Service Worker и кэшируем критические файлы
self.addEventListener('install', event => {
  self.skipWaiting();
  event.waitUntil(
    caches.open(cacheName).then(cache => {
      return cache.addAll([
        './', // кэширование корня
        './index.html', // кэширование HTML
        './manifest.json', // манифест
        './service_worker.js', // сам service worker
        './icon-192x192.png', // иконка
        './icon-512x512.png', // большая иконка
      ]);
    })
  );
});

// Обрабатываем запросы и динамически кэшируем все файлы
self.addEventListener('fetch', event => {
  event.respondWith(
    caches.match(event.request).then(cachedResponse => {
      // Если ресурс есть в кэше, возвращаем его, и обновляем кэш в фоне
      if (cachedResponse) {
        fetch(event.request).then(networkResponse => {
          caches.open(cacheName).then(cache => {
            cache.put(event.request, networkResponse.clone());
          });
        }).catch(err => {
          console.error('Network fetch failed for:', event.request.url, err);
        });
        return cachedResponse;
      }

      // Если ресурса нет в кэше, делаем сетевой запрос и кэшируем его
      return fetch(event.request).then(networkResponse => {
        return caches.open(cacheName).then(cache => {
          cache.put(event.request, networkResponse.clone());
          return networkResponse;
        });
      }).catch(err => {
        console.error('Network request failed for:', event.request.url, err);
      });
    })
  );
});

// Очистка старого кэша при активации нового Service Worker
self.addEventListener('activate', event => {
  const cacheWhitelist = [cacheName];
  event.waitUntil(
    caches.keys().then(cacheNames => {
      return Promise.all(
        cacheNames.map(cache => {
          if (!cacheWhitelist.includes(cache)) {
            console.log('Deleting old cache:', cache);
            return caches.delete(cache);
          }
        })
      );
    }).then(() => self.clients.claim())
  );
});
