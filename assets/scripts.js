"use strict";

// a simple console message to confirm that scripts.js is loaded and working
console.log("scripts.js loaded");

// add a hamburger menu to control navbar visibility on small screens
const hamburger = document.querySelector(".hamburger");
const navbarItems = document.querySelector(".navbarItems");

function toggleVisible() {
  navbarItems.classList.toggle("active");
}

hamburger.addEventListener("click", toggleVisible);
