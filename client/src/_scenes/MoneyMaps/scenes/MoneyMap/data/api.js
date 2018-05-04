import Auth from '~/_helpers/auth';

/**
 * Accounts API
 */

class AccountsApi {

	/*
	*	Get Accounts for money map
	*/
	static getAccounts() {
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
}

export default AccountsApi;