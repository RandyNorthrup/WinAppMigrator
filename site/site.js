// Shows the list of pages beside the text on a wide screen, where there is room for it, and draws the diagrams on the
// pages that have them. Every page reads fine without it: the list opens with a click, and a diagram stays as its text.
(function () {
  "use strict";

  var sidebar = document.getElementById("sidebar");
  var wide = window.matchMedia("(min-width: 960px)");

  function fit() {
    if (sidebar) {
      sidebar.open = wide.matches;
    }
  }

  fit();
  wide.addEventListener("change", fit);

  if (window.mermaid && document.querySelector("pre.mermaid")) {
    var dark = window.matchMedia("(prefers-color-scheme: dark)").matches;
    window.mermaid.initialize({ startOnLoad: false, securityLevel: "strict", theme: dark ? "dark" : "default" });
    window.mermaid.run({ querySelector: "pre.mermaid" });
  }
})();
