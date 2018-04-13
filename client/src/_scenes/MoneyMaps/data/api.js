import Auth from '~/_helpers/auth';

/**
 * Money Maps API
 */

class MoneyMapsApi {

	static getMoneyMaps() {
		return fetch(
			'/money_maps',
			{
				method: 'GET',
				headers: {
					'Authorization': Auth.getAuthHeader(),
					'Content-Type': 'application/json'
				}
			}
		).then(response => {
			switch (response.status) {
				case 200:
					return response.json();
				default:
					return new Error('request failed')
			}
		}).catch(error => {
			return error;
		});
	}

	static updateAccount({firstName, lastName}) {
		let body = {
			first_name: firstName,
			last_name: lastName
		}
		return fetch(
			'http://localhost:3000/account',
			{
				method: 'PATCH',
				headers: {
					'Authorization': Auth.getAuthHeader(),
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(body)
			}
		).then(response => {
			switch (response.status) {
				case 200:
					return response.json();
				default:
					return new Error('request failed')
			}
		}).catch(error => {
			return error;
		});
	}
}

export default MoneyMapsApi;