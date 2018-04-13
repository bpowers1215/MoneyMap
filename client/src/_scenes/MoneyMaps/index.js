import React, { Component } from 'react';
import { connect } from 'react-redux';
import Alerts from '~/_components/alerts';
import MoneyMapsActions from './data/actions';
import MoneyMapsList from './components/MoneyMapsList';
import './styles.scss';

const mapDispatchToProps = dispatch => {
	return {
		getMoneyMaps: () => dispatch(MoneyMapsActions.getMoneyMaps())
	};
};

const mapStateToProps = state => {
	let moneyMaps = state.scenes.moneyMaps.data.moneyMaps;
	return {
		moneyMaps: moneyMaps
	}
}

class ConnectedMoneyMaps extends Component {
	constructor(props) {
		super(props);
		this.state = {};
	}
	componentDidMount(){
		this.props.getMoneyMaps();
	}
	render() {
		return (
			<div>
				<div className="hero is-primary">
					<div className="hero-body">
						<div className="container">
							<h1 className="title">
								My Money Maps
							</h1>
						</div>
					</div>
				</div>
				<div className="container is-fluid page-content">
					<Alerts />
					<MoneyMapsList moneyMaps={this.props.moneyMaps} />
				</div>
			</div>
		);
	}
}

const MoneyMaps = connect(mapStateToProps, mapDispatchToProps)(ConnectedMoneyMaps);
export default MoneyMaps;
