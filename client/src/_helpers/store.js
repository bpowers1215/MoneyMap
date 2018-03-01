import { createStore, applyMiddleware } from 'redux';
import thunk from 'redux-thunk';
import { enableBatching } from 'redux-batched-actions';
import Cookies from 'js-cookie';
import { createCookieMiddleware } from 'redux-cookie';
import rootReducer from '../_reducers/reducer';

const store = createStore(
	enableBatching(rootReducer),
	applyMiddleware(
		thunk,
		createCookieMiddleware(Cookies)
	)
	
);
export default store;