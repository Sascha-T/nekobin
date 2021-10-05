/*
//  Copyright 2021 neko.rs contributors <https://neko.rs>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

function infiniteScroll(url, trigger) {
  return {
    triggerElement: null,
    page: 0,
    observer: null,
    items: [],
    init(elementId) {
      const ctx = this
      this.triggerElement = document.querySelector(elementId ? elementId : trigger)
      this.observer = new IntersectionObserver(function(entries) {
        if(entries[0].isIntersecting === true) {
          ctx.loadMore()
        }
      }, { threshold: [0] })
      this.observer.observe(this.triggerElement)
    },
    loadMore() {
      fetch(url + this.page).then(response => response.json()).then(data => {
        if(data.length > 0) {
          this.items = this.items.concat(data)
          this.page++
        } else {
          this.observer.unobserve(this.triggerElement)
          this.triggerElement.parentNode.removeChild(this.triggerElement)
        }
      });
    }
  }
}

// https://stackoverflow.com/a/3177838
function timeSince(date) {

  var seconds = Math.floor((new Date() - date) / 1000);

  var interval = seconds / 31536000;

  if (interval > 1) {
    return Math.floor(interval) + " years";
  }
  interval = seconds / 2592000;
  if (interval > 1) {
    return Math.floor(interval) + " months";
  }
  interval = seconds / 86400;
  if (interval > 1) {
    return Math.floor(interval) + " days";
  }
  interval = seconds / 3600;
  if (interval > 1) {
    return Math.floor(interval) + " hours";
  }
  interval = seconds / 60;
  if (interval > 1) {
    return Math.floor(interval) + " minutes";
  }
  return Math.floor(seconds) + " seconds";
}

function toggleTheme() {
  if (localStorage.getItem('light_mode') === 'true') {
    localStorage.setItem('light_mode', false);
    document.documentElement.classList.add('dark');
  } else {
    localStorage.setItem('light_mode', true);
    document.documentElement.classList.remove('dark');
  }
}

function discordAvatar(u) {
  var url = "https://cdn.discordapp.com/";
  if (u.avatar) {
    url+="avatars/"+u.id+"/"+u.avatar;
    if (u.avatar.startsWith("a_")) {
      return url+".gif"
    } else {
      return url+".png"
    }
  } else {
    return url+"embed/avatars/"+u.discriminator%5+".png";
  }
}