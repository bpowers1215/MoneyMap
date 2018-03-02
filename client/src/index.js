import React from 'react';
import ReactDOM from 'react-dom';
import store from "~/_helpers/store";
import Root from '~/_components/Root';
import registerServiceWorker from '~/registerServiceWorker';
import '~/index.css';
import 'bootstrap/dist/css/bootstrap.css';

ReactDOM.render(
	<Root store={store} />
	, document.getElementById('root')
);
registerServiceWorker();
