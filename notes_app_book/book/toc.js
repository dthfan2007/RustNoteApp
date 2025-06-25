// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="introduction.html"><strong aria-hidden="true">1.</strong> Introduction</a></li><li class="chapter-item expanded "><a href="planning.html"><strong aria-hidden="true">2.</strong> Planning</a></li><li class="chapter-item expanded "><a href="main/main.html"><strong aria-hidden="true">3.</strong> Main Content</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="main/procedure.html"><strong aria-hidden="true">3.1.</strong> Procedure</a></li><li class="chapter-item expanded "><a href="main/fn_descriptions.html"><strong aria-hidden="true">3.2.</strong> Function Descriptions</a></li></ol></li><li class="chapter-item expanded "><a href="testing.html"><strong aria-hidden="true">4.</strong> Testing</a></li><li class="chapter-item expanded "><a href="dailies.html"><strong aria-hidden="true">5.</strong> Dailies</a></li><li class="chapter-item expanded "><a href="appendix/appendix.html"><strong aria-hidden="true">6.</strong> Appendix</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="appendix/sources.html"><strong aria-hidden="true">6.1.</strong> Sources</a></li><li class="chapter-item expanded "><a href="appendix/glossary.html"><strong aria-hidden="true">6.2.</strong> Glossary</a></li></ol></li><li class="chapter-item expanded "><a href="snippets.html"><strong aria-hidden="true">7.</strong> Code Snippets</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="snippets/snippet_1.html"><strong aria-hidden="true">7.1.</strong> Snippet 1</a></li><li class="chapter-item expanded "><a href="snippets/snippet_2.html"><strong aria-hidden="true">7.2.</strong> Snippet 2</a></li><li class="chapter-item expanded "><a href="snippets/snippet_3.html"><strong aria-hidden="true">7.3.</strong> Snippet 3</a></li><li class="chapter-item expanded "><a href="snippets/snippet_4.html"><strong aria-hidden="true">7.4.</strong> Snippet 4</a></li><li class="chapter-item expanded "><a href="snippets/snippet_5.html"><strong aria-hidden="true">7.5.</strong> Snippet 5</a></li><li class="chapter-item expanded "><a href="snippets/snippet_6.html"><strong aria-hidden="true">7.6.</strong> Snippet 6</a></li><li class="chapter-item expanded "><a href="snippets/snippet_7.html"><strong aria-hidden="true">7.7.</strong> Snippet 7</a></li><li class="chapter-item expanded "><a href="snippets/snippet_8.html"><strong aria-hidden="true">7.8.</strong> Snippet 8</a></li><li class="chapter-item expanded "><a href="snippets/snippet_9.html"><strong aria-hidden="true">7.9.</strong> Snippet 9</a></li><li class="chapter-item expanded "><a href="snippets/snippet_10.html"><strong aria-hidden="true">7.10.</strong> Snippet 10</a></li><li class="chapter-item expanded "><a href="snippets/snippet_11.html"><strong aria-hidden="true">7.11.</strong> Snippet 11</a></li><li class="chapter-item expanded "><a href="snippets/snippet_12.html"><strong aria-hidden="true">7.12.</strong> Snippet 12</a></li><li class="chapter-item expanded "><a href="snippets/snippet_13.html"><strong aria-hidden="true">7.13.</strong> Snippet 13</a></li><li class="chapter-item expanded "><a href="snippets/snippet_14.html"><strong aria-hidden="true">7.14.</strong> Snippet 14</a></li><li class="chapter-item expanded "><a href="snippets/snippet_15.html"><strong aria-hidden="true">7.15.</strong> Snippet 15</a></li><li class="chapter-item expanded "><a href="snippets/snippet_16.html"><strong aria-hidden="true">7.16.</strong> Snippet 16</a></li><li class="chapter-item expanded "><a href="snippets/snippet_17.html"><strong aria-hidden="true">7.17.</strong> Snippet 17</a></li><li class="chapter-item expanded "><a href="snippets/snippet_18.html"><strong aria-hidden="true">7.18.</strong> Snippet 18</a></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
