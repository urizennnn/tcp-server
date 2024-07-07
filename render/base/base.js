console.log('base.js loaded');

document.getElementById('receiver').addEventListener('click', function(e) {
	e.preventDefault();

	fetch('http://localhost:8080/receiver', {
		method: 'GET',
	})
		.then(response => {
			if (!response.ok) {
				throw new Error('Network response was not ok');
			}
			return response.text();
		})

		.catch(error => console.error('Error:', error));
});

document.getElementById('sender').addEventListener('click', function(e) {
	e.preventDefault();

	fetch('http://localhost:8080/sender', {
		method: 'GET',
	})
		.then(response => {
			if (!response.ok) {
				throw new Error('Network response was not ok');
			}
			return response.text();
		})

		.catch(error => console.error('Error:', error));
});

