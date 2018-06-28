import React, { Component } from 'react';
import { connect } from 'react-redux';
import Alerts from '~/_components/alerts';
import MoneyMapActions from '~/_scenes/MoneyMaps/data/actions';
import MoneyMapAccountActions from './data/actions';
import AccountDetails from './components/AccountDetails';
import { history } from '~/_helpers/history';
import './styles.scss';

const mapDispatchToProps = dispatch => {
	return {
		getMoneyMap: () => dispatch(MoneyMapActions.getMoneyMaps()),
		cantFindMoneyMap: () => {
			dispatch(MoneyMapActions.cantFindMoneyMap('/money_maps'));
			history.push('/money_maps');
		},
		getAccounts: () => dispatch(MoneyMapAccountActions.getAccounts()),
	};
};

const mapStateToProps = state => {
	let moneyMaps = state.scenes.moneyMaps.data.moneyMaps;
	return {
		moneyMaps: moneyMaps
	}
}

class ConnectedMoneyMapAccount extends Component {
	constructor(props) {
		super(props);
		this.state = {
			moneyMap: {
				id: '',
				name: ''
			},
			account: {
				id: '',
				name: ''
			}
		};
	}
	componentWillMount(){
		if ( !this.moneyMapPresent(this.props.moneyMaps, this.props.match.params.moneyMapId) ) {
			this.props.getMoneyMap();
		} else {
			this.selectMoneyMap(this.props.moneyMaps, this.props.match.params.moneyMapId, this.props.match.params.accountId);
		}
	}
	componentWillReceiveProps(nextProps){
		let { match, moneyMaps } = nextProps;
		let moneyMapId = match.params.moneyMapId;
		let accountId = match.params.accountId;
		this.selectMoneyMap(moneyMaps, moneyMapId, accountId)
	}
	shouldComponentUpdate(nextProps, nextState) {
		let { match, moneyMaps } = nextProps;
		let moneyMapId = match.params.moneyMapId;

		if ( !this.moneyMapPresent(moneyMaps, moneyMapId) ) {
			this.props.cantFindMoneyMap();
			return false;
		}
		return true;
	}
	moneyMapPresent(moneyMaps, moneyMapId) {
		return moneyMaps && moneyMapId && moneyMapId in moneyMaps;
	}
	selectMoneyMap(moneyMaps, moneyMapId, accountId) {
		let newState = {
			...this.state,
			moneyMap: moneyMaps[moneyMapId],
			account: moneyMaps[moneyMapId].accounts[accountId]
		}
		this.setState(newState);
	}
	render() {
		return (
			<div>
				<div className="hero is-primary">
					<div className="hero-body">
						<div className="container">
							<h1 className="title">
								Money Map Account
							</h1>
						</div>
					</div>
				</div>
				<div className="container is-fluid page-content">
					<Alerts />
					<AccountDetails account={this.state.account} moneyMap={this.state.moneyMap} />
				</div>
			</div>
		);
	}
}

const MoneyMapAccount = connect(mapStateToProps, mapDispatchToProps)(ConnectedMoneyMapAccount);
export { MoneyMapAccount };
