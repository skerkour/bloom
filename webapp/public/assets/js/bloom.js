(function(){

  var hostname = window.location.hostname;
  var analytics = false;
  if (hostname === 'bloom.sh') {
    analytics = true;
  }

  window.$bloom = {
    project: '01776890-92c0-a614-3490-25d136ee2abe',
    analytics: analytics,
    chatbox: false,
  };

  d=document;s=d.createElement("script");s.src="https://bloom.sh/libs/bloom.js"; s.async=1;d.getElementsByTagName("head")[0].appendChild(s);

})();
