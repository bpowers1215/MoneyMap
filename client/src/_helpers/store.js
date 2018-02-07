import { createStore, applyMiddleware } from 'redux';
import thunk from 'redux-thunk';
import { enableBatching } from 'redux-batched-actions';
import rootReducer from '../_reducers/reducer';

const store = createStore(
	enableBatching(rootReducer),
	applyMiddleware(
		thunk
	)
	
);
window.store = store;
export default store;