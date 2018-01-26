import { createStore, applyMiddleware } from 'redux';
import thunk from 'redux-thunk';
import rootReducer from '../_reducers/reducer';

const store = createStore(
	rootReducer,
	applyMiddleware(
		thunk
	)
);
window.store = store;
export default store;