/*
  all properties / methods starting with `this.` are not mangled.
*/
(function(window) {
  "use strict";
  var blmNavigator = navigator || {};
  var log = console.log
  var logDebug = console.debug;
  var logWarn = console.warn;
  var logError = console.error;
  var blmLocalStorage = window.localStorage;
  var jsonStringify = JSON.stringify;
  var jsonParse = JSON.parse;
  var isBotAgent = function(userAgent) { return /(bot|spider|crawl)/i.test(userAgent) && !/(cubot)/i.test(userAgent) };
  var isDoNotTrackEnabled = blmNavigator.doNotTrack === "1";

  // constants
  var bloomChatboxElemId = 'bloom-chatbox';
  var defaultBaseUrl = 'https://bloom.sh';
  var storageKey = '__bloom';

  // global vars
  var lastPage = '';
  var anonymousId = getAnonymousId();

  // options / config
  var bloomConfig = window.$bloom || {};
  bloomConfig.url = bloomConfig.url || defaultBaseUrl;
  bloomConfig.chatbox = bloomConfig.chatbox === false ? false : true;
  bloomConfig.analytics = bloomConfig.analytics === false ? false : (true && !isDoNotTrackEnabled);
  bloomConfig.anonymous_id = anonymousId;

  // validate config
  if (!bloomConfig.project) {
    logError('bloom: project not configured');
    return;
  }

  function getAnonymousId() {
    var stored = blmLocalStorage.getItem(storageKey);
    if (!stored) {
      var data = {
        anonymous_id:  uuidv4(),
      };
      blmLocalStorage.setItem(storageKey, jsonStringify(data));
      return data.anonymous_id;
    } else {
      var data = jsonParse(stored);
      return data.anonymous_id;
    }
  }

  // TODO: better function (crypto random)
  function uuidv4() {
    var uuid = "", i, random;
    for (i = 0; i < 32; i++) {
      random = Math.random() * 16 | 0;

      if (i == 8 || i == 12 || i == 16 || i == 20) {
        uuid += "-"
      }
      uuid += (i == 12 ? 4 : (i == 16 ? (random & 3 | 8) : random)).toString(16);
    }
    return uuid;
  }

  function chatbox() {
    var chatboxUrl = bloomConfig.url + '/libs/chatbox.js';
    // load chatbox widget
    var chatboxElem = document.createElement("div");
    chatboxElem.id = bloomChatboxElemId;
    chatboxElem.style.display = 'none';
    document.getElementsByTagName("body")[0].appendChild(chatboxElem);
    var scriptElem = document.createElement("script");
    scriptElem.src = chatboxUrl;
    scriptElem.type = "text/javascript";
    scriptElem.async = 1;
    document.getElementsByTagName("head")[0].appendChild(scriptElem);
  }

  function sendEvent(event, endpoint) {
    fetch(endpoint, {
      method: 'POST',
      body: jsonStringify(event),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Anonymous ' + anonymousId,
      },
    })
    .catch(function(err){ logError('Bloom: ', err); });
  }

  function trackPage() {
    var pageEventEndpoint = bloomConfig.url + '/api/a/events/p';
    lastPage = window.location.pathname;

    var now = new Date().toISOString();
    var event = {
      anonymous_id: anonymousId, // will be overwritten server side
      timestamp: now,
      sent_at: now,
      received_at: now, // will be overwritten server side
      namespace_id: bloomConfig.project,
      name: document.title,
      url: window.location.href,
      referrer: document.referrer,
      screen_width: screen.width || 0,
      screen_height: screen.height || 0,
      user_agent: blmNavigator.userAgent,
    };

    if (isBotAgent(event.user_agent)) {
      return;
    }

    sendEvent(event, pageEventEndpoint);
  }

  function startTrackPages() {
    trackPage();

    // interval for SPA
    setInterval(function() {
      if (lastPage !== window.location.pathname) {
        trackPage();
      }
    }, 800);
  }

  if (bloomConfig.chatbox) {
    chatbox();
  }

  if (bloomConfig.analytics) {
    startTrackPages();
  }
})(window)
