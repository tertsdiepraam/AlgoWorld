function openTab(id, bttn) {
	var i, tabcontent, tablinks;
	tabcontent = document.getElementsByClassName("tabcontent");
	for (i = 0; i < tabcontent.length; i++) {
		tabcontent[i].style.display = "none";
	}

	tablinks = document.getElementsByClassName("tablink");
	for (i = 0; i < tabcontent.length; i++) {
		tablinks[i].style.backgroundColor = "#dddddd";
	}

	document.getElementById(id).style.display = "block";
	bttn.style.backgroundColor = "#ffffff"
}

document.getElementById("defaultOpen").click();
