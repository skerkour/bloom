(function(){

  var hostname = window.location.hostname;
  var analytics = false;
  if (hostname === 'bloom.sh') {
    analytics = true;
  }

  window.$bloom = {
    project: 'e77fcbde-b179-42f4-977e-93ffaa1bc6e0',
    analytics: analytics,
  };

  d=document;s=d.createElement("script");s.src="https://bloom.sh/libs/bloom.js"; s.async=1;d.getElementsByTagName("head")[0].appendChild(s);

})();
