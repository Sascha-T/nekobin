/*
//  Copyright 2021 neko.rs contributors <https://neko.rs>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/


var player = document.getElementById('player');
var video = player.getElementsByTagName("video")[0];
var overlay = player.getElementsByTagName("div")[0];

var progress_bar = document.getElementById('progress_bar');
var playpause_button = document.getElementById('playpause');
var fullscreen_button = document.getElementById('toggle_fullscreen');

var autohide_timeout_id = null;

progress_bar.value = 0;

function clear_autohide_timeout() {
    if (autohide_timeout_id != null) {
        clearTimeout(autohide_timeout_id);
        autohide_timeout_id = null;
    }
}

video.addEventListener('play', function(ev) {
    playpause_button.classList.remove('mdi-play');
    playpause_button.classList.add('mdi-pause');
});

player.addEventListener('mouseenter', function(e) {
    showOverlay()
    if (!(video.paused || video.ended)) {
        clear_autohide_timeout();
        autohide_timeout_id = setTimeout(function() {
            hideOverlay();
        }, 3000)
    }
});

player.addEventListener('mousemove', function(e) {
    showOverlay();
    clear_autohide_timeout();
    if (!(video.paused || video.ended)) {
        autohide_timeout_id = setTimeout(function() {
            hideOverlay();
        }, 3000)
    }
});

player.addEventListener('mouseleave', function(e) {
    if (!(video.paused || video.ended)) {
        hideOverlay()
        clear_autohide_timeout()
    }
});

video.addEventListener('ended', function(e) {
    clear_autohide_timeout()
    playpause_button.classList.remove('mdi-pause');
    playpause_button.classList.add('mdi-play');
    showOverlay();
});

video.addEventListener('pause', function(e) {
    clear_autohide_timeout()
    playpause_button.classList.remove('mdi-pause');
    playpause_button.classList.add('mdi-play');

    showOverlay();
});

video.addEventListener('loadedmetadata', function() {
    progress_bar.setAttribute('max', Math.floor(video.duration*1000));
});

video.addEventListener('timeupdate', function() {
    if (!progress_bar.getAttribute('max')) progress_bar.setAttribute('max', Math.floor(video.duration*1000));
    progress_bar.value = Math.floor(video.currentTime*1000);
})

function playVideo() {
    if (video.paused || video.ended) {
        video.play();
    }
    else {
        video.pause();
    }
}

function setVideo(src, restoreTime) {
    let paused = video.paused || video.ended;
    let sas = video.currentTime;
    video.src = src;
    video.load();
    if (restoreTime) {
        video.currentTime = sas;
    }
    if (!paused && restoreTime) {
        video.play();
    }
}

function fullscreen() {
    if (document.fullscreenElement == player) {
        document.exitFullscreen();
    }
    else {
        player.requestFullscreen();
    }
}

document.addEventListener('keydown', function(e) {
    if (e.key == "f") {
        e.preventDefault();
        fullscreen();
    }
    else if (e.key == "ArrowRight") {
        e.preventDefault();
        video.currentTime += 5;
    }
    else if (e.key == "ArrowLeft") {
        e.preventDefault();
        video.currentTime -= 5;
    }
    else if (e.key == " ") {
        e.preventDefault();
        playVideo();
    }
});

function hideOverlay() {
    player.style.cursor = "none";
    overlay.classList.add('hidden');
}

function showOverlay() {
    player.style.cursor = "initial";
    overlay.classList.remove('hidden');
}

progress_bar.addEventListener('input', function(e) {
    video.currentTime = this.value/1000;
});

player.addEventListener('fullscreenchange', function(e) {
    if (document.fullscreenElement == player) {
        fullscreen_button.classList.remove('mdi-fullscreen');
        fullscreen_button.classList.add('mdi-fullscreen-exit');
    }
    else {
        fullscreen_button.classList.remove('mdi-fullscreen-exit');
        fullscreen_button.classList.add('mdi-fullscreen');
    }
});