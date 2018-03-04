import React from 'react';
import ReactDOM from 'react-dom';
import store from "~/_helpers/store";
import Root from '~/_components/Root';
import registerServiceWorker from '~/registerServiceWorker';
//import '~/styles/index.scss';
//import 'bootstrap/dist/css/bootstrap.css';

ReactDOM.render(
	<Root store={store} />
	, document.getElementById('root')
);
registerServiceWorker();
