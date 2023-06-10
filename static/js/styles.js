document.addEventListener("DOMContentLoaded", function() {
	var blockquotes = document.getElementsByTagName("blockquote");
	
	for (var i = 0; i < blockquotes.length; i++) {
		var blockquote = blockquotes[i];
		var strong     = blockquote.querySelector("strong");
		
		//	Check the text content and add the class if it matches
		if (strong && !strong.textContent.includes(" ")) {
			blockquote.classList.add(strong.textContent.replace(/\W/g, "").toLowerCase());
		}
	}
});


