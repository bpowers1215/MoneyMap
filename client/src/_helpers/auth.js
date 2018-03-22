import { globalConstants } from '~/_constants';
import Cookies from 'js-cookie';

/* 
* Check if the user is authenticated by looking for the auth token in cookies
* TODO: Check if the user is authenticated with the API using the auth token
*/
const isAuthenticated = () => {
	let cookie = Cookies.get(globalConstants.AUTH_TOKEN_COOKIE);
	return !!cookie;
}

/* 
* Get Authentication Header for API requests
*/
const getAuthHeader = () => {
	return "Bearer " + getAuthToken();
}

/* 
* Get Auth token from cookie
*/
const getAuthToken = () => {
	return Cookies.get(globalConstants.AUTH_TOKEN_COOKIE);
}

const auth = {
	isAuthenticated,
	getAuthHeader
}

export default auth;