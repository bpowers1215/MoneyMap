/**
 * Users API
 */

class UsersApi {

	static login(email, password) {
		let body = {
			email: email,
			password: password
		}
		return fetch(
			'http://localhost:3000/account/login',
			{
				method: 'POST',
				header: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(body)
			}
		).then(response => {
			switch (response.status) {
				case 200:
					return response.json();
				default:
					return new Error('failed to login')
			}
		}).catch(error => {
			return error;
		});
	}
}

export default UsersApi;