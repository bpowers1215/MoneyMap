import React, { Component } from 'react';
import { connect } from 'react-redux';
import Alerts from '~/_components/alerts';
import AddMoneyMapForm from './components/AddMoneyMapForm';
import './styles.scss';

const mapDispatchToProps = dispatch => {
	return {
	};
};

const mapStateToProps = state => {
	return {}
}

class ConnectedAddMoneyMap extends Component {
	constructor(props) {
		super(props);
		this.state = {};
	}
	componentDidMount(){
	}
	render() {
		return (
			<div>
				<div className="hero is-primary">
					<div className="hero-body">
						<div className="container">
							<h1 className="title">
								Add Money Map
							</h1>
						</div>
					</div>
				</div>
				<div className="container is-fluid page-content">
					<Alerts />
					<AddMoneyMapForm />
				</div>
			</div>
		);
	}
}

const AddMoneyMap = connect(mapStateToProps, mapDispatchToProps)(ConnectedAddMoneyMap);
export { AddMoneyMap };
