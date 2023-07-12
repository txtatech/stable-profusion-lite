use web_view::*;
use serde_json::Value;

fn main() {
    let html = r#"
<html>
   <head>
      <style>
         body.dark-mode {
         background-color: #111;
         color: #fff;
         }
         body.dark-mode a {
         color: #fff;
         }
         body.dark-mode .separator {
         color: #999;
         }
         body.dark-mode .versions {
         color: #ff9800;
         }
         /* Specific dark mode styles for iframe */
         #ssb-container.dark-mode iframe {
         filter: invert(1);
         }
         table {
         border-collapse: collapse;
         }
         table td {
         padding: 2px;
         }
         input[type="text"] {
         background-color: #333;
         color: #fff;
         border: none;
         padding: 5px;
         }
         button {
         background-color: #444;
         color: #fff;
         border: none;
         padding: 2px 2px;
         cursor: pointer;
         }
         textarea {
         background-color: #333;
         color: #fff;
         border: none;
         padding: 2px;
         resize: none;
         font-size: 1.0em;
         width: 100%;
         }
         .ssb-wrapper {
         transform: scale(.99);
         position: relative; /* Set the position of the wrapper to relative */
         margin-bottom: 10px;
         margin-left: 1px;
         margin-right: 1px;
         margin-top: 10px;
         }
      </style>
   </head>
   <body class="dark-mode">
      <table>
         <tr>
            <td>
               <input type="text" id="url-input1" placeholder="Enter URL1" value="http://127.0.0.1:7860">
               <button onclick="openNewWebView('url-input1')">Web</button>
               <button onclick="loadSSB('url-input1')">SSB</button>
               <input type="text" id="url-input" placeholder="Enter URL" value="http://127.0.0.1:8989">
               <button onclick="openNewWebView('url-input')">Web</button>
               <button onclick="loadSSB('url-input')">SSB</button>
            </td>
            <td><button onclick="reload()">Reload</button></td>
            <td><button onclick="stop()">Stop</button></td>
            <td><button onclick="goBack()">Back</button></td>
            <td><button onclick="goForward()">Forward</button></td>
         <td><button onclick="toggleFullscreen()">Fullscreen</button></td>
         <td><button onclick="inspectElement()">Inspect</button></td>
         <td><select id="url-history" onchange="loadURLFromHistory()">
         <option value="">History</option>
         </select>
          </td>
         <td><a href="http://127.0.0.1:7860">SD-7860</a>
         <a href="http://127.0.0.1:7861">SD-7861</a>
         <a href="http://127.0.0.1:8989">SD-8989</a></td>
         <td><input type="text" id="search-input" placeholder="Enter search query"></td>
         <td><button onclick="conductSearch()">Search</button></td>
         </tr>
      </table>
      <div id="ssb-container"></div>
      <h6>+ Prompt:</h6>
      <textarea rows="10" cols="141" style="font-size: 1.0em;"></textarea>
      <h6>- Prompt:</h6>
      <textarea rows="10" cols="141" style="font-size: 1.0em;"></textarea>
      <h6>Scratchpad:</h6>
      <textarea rows="10" cols="141" style="font-size: 1.0em;"></textarea>
      <script>
         function conductSearch() {
           const searchInput = document.getElementById('search-input');
           const query = searchInput.value.trim();

           if (query !== '') {
             const baseUrl = 'https://search.brave.com'; // Hardcoded URL
             const searchUrl = baseUrl + '/search?q=' + encodeURIComponent(query);
             window.location.href = searchUrl;
           }
         }

      </script>
      <script>
         let urlHistory = [];

         function openNewWebView(inputId) {
             const urlInput = document.getElementById(inputId);
             const url = urlInput.value.trim();

             if (url !== '') {
                 if (inputId === 'url-input' || inputId === 'url-input1') {
                     // Update the URL in the current web view using JavaScript
                     window.location.href = url;
                 } else {
                     // Open the URL in a new web view
                     window.open(url);
                 }
                 // Add the URL to the history array
                 addToUrlHistory(url);
             }
         }

         function addToUrlHistory(url) {
             // Check if the URL is already in the history
             const index = urlHistory.indexOf(url);
             if (index !== -1) {
                 // If the URL exists, remove it from its current position
                 urlHistory.splice(index, 1);
             }
             // Add the URL at the beginning of the history array
             urlHistory.unshift(url);
             // Update the select element with the new history
             updateUrlHistorySelect();
         }

         function updateUrlHistorySelect() {
             const urlHistorySelect = document.getElementById('url-history');
             // Clear the select element
             urlHistorySelect.innerHTML = '<option value="">History</option>';
             // Add options for each URL in the history array
             for (let i = 0; i < urlHistory.length; i++) {
                 const url = urlHistory[i];
                 const option = document.createElement('option');
                 option.value = url;
                 option.text = url;
                 urlHistorySelect.appendChild(option);
             }
         }

         function loadURLFromHistory() {
             const urlHistorySelect = document.getElementById('url-history');
             const selectedUrl = urlHistorySelect.value;
             if (selectedUrl !== '') {
                 // Update the URL input field with the selected URL
                 const urlInput = document.getElementById('url-input');
                 urlInput.value = selectedUrl;
             }
         }
      </script>
      <script>
         function openNewWebView(inputId) {
         const urlInput = document.getElementById(inputId);
         const url = urlInput.value.trim();

         if (url !== '') {
         if (inputId === 'url-input' || inputId === 'url-input1') {
         // Update the URL in the current web view using JavaScript
         window.location.href = url;
         } else {
         // Open the URL in a new web view
         window.open(url);
         }
         }
         }

         // Function to load SSB (Single Site Browser)
         function loadSSB(inputId) {
         const ssbUrlInput = document.getElementById(inputId);
         const ssbUrl = ssbUrlInput.value;
         const ssbContainer = document.getElementById('ssb-container');
         const ssbWrapper = document.createElement('div');
         ssbWrapper.className = 'ssb-wrapper'; // Add the CSS class for styling
         const iframe = document.createElement('iframe');
         iframe.src = ssbUrl;
         iframe.style.width = '97%';
         iframe.style.height = '120%';
         const urlCloseContainer = document.createElement('div');
         urlCloseContainer.className = 'url-close-container'; // Add the CSS class for styling
         const closeButton = document.createElement('button');
         closeButton.innerHTML = `Close ${ssbUrl}`;
         closeButton.onclick = function () {
           ssbWrapper.remove();
         };
         ssbWrapper.appendChild(iframe);
         urlCloseContainer.appendChild(closeButton);
         ssbWrapper.appendChild(urlCloseContainer);
         ssbContainer.appendChild(ssbWrapper); // Append ssbWrapper to ssbContainer
         ssbUrlInput.value = '';
         ssbUrlInput.value = ssbUrl;
         }

         function toggleFullscreen() {
         const currentWindow = window.parent;
         if (currentWindow.document.documentElement.requestFullscreen) {
           currentWindow.document.documentElement.requestFullscreen();
         } else if (currentWindow.document.documentElement.mozRequestFullScreen) {
           currentWindow.document.documentElement.mozRequestFullScreen();
         } else if (currentWindow.document.documentElement.webkitRequestFullscreen) {
           currentWindow.document.documentElement.webkitRequestFullscreen();
         } else if (currentWindow.document.documentElement.msRequestFullscreen) {
           currentWindow.document.documentElement.msRequestFullscreen();
         }
         }

         function goBack() {
         window.history.back();
         }

         function goForward() {
         window.history.forward();
         }

         function stop() {
         const currentWindow = window.parent;
         currentWindow.stop();
         }

         function reload() {
         const currentWindow = window.parent;
         currentWindow.location.reload();
         }

         function inspectElement() {
         const currentWindow = window.parent;
         currentWindow.document.documentElement.classList.add('inspect-element');
         }

      </script>
   </body>
</html>
    "#;

    let webview = web_view::builder()
        .title("Stable-Profusion-Lite")
        .content(Content::Html(html))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
            if let Ok(args) = serde_json::from_str::<Vec<Value>>(arg) {
                if let (Some(function_name), Some(url)) = (args.get(0).and_then(Value::as_str), args.get(1).and_then(Value::as_str)) {
                    if function_name == "Web" {
                        // Execute JavaScript to update the URL in the main web view
                        webview.eval(&format!("window.location.href = '{}';", url))?;
                    }
                }
            }
            Ok(())
        })
        .build()
        .unwrap();

    webview.run().unwrap();
}