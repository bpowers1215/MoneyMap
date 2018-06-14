import React, { Component } from 'react';
import { connect } from 'react-redux';
import Alerts from '~/_components/alerts';
import MoneyMapActions from '~/_scenes/MoneyMaps/data/actions';
import AccountsActions from './data/actions';
import AccountsList from './components/AccountsList';
import { history } from '~/_helpers/history';
import './styles.scss';

const mapDispatchToProps = dispatch => {
	return {
		getMoneyMap: () => dispatch(MoneyMapActions.getMoneyMaps()),
		cantFindMoneyMap: () => {
			dispatch(MoneyMapActions.cantFindMoneyMap('/money_maps'));
			history.push('/money_maps');
		},
		getAccounts: () => dispatch(AccountsActions.getAccounts()),
	};
};

const mapStateToProps = state => {
	let moneyMaps = state.scenes.moneyMaps.data.moneyMaps;
	return {
		moneyMaps: moneyMaps
	}
}

class ConnectedMoneyMap extends Component {
	constructor(props) {
		super(props);
		this.state = {
			moneyMap: {
				id: '',
				name: ''
			}
		};
	}
	componentWillMount(){
		if ( !this.moneyMapPresent(this.props.moneyMaps, this.props.match.params.id) ) {
			this.props.getMoneyMap();
		} else {
			this.selectMoneyMap(this.props.moneyMaps, this.props.match.params.id);
		}
	}
	componentWillReceiveProps(nextProps){
		let { match, moneyMaps } = nextProps;
		let moneyMapId = match.params.id;
		this.selectMoneyMap(moneyMaps, moneyMapId)
	}
	shouldComponentUpdate(nextProps, nextState) {
		let { match, moneyMaps } = nextProps;
		let moneyMapId = match.params.id;

		if ( !this.moneyMapPresent(moneyMaps, moneyMapId) ) {
			this.props.cantFindMoneyMap();
			return false;
		}
		return true;
	}
	moneyMapPresent(moneyMaps, moneyMapId) {
		return moneyMaps && moneyMapId && moneyMapId in moneyMaps;
	}
	selectMoneyMap(moneyMaps, moneyMapId) {
		let newState = {
			moneyMap: moneyMaps[moneyMapId]
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
								{this.state.moneyMap.name}
							</h1>
						</div>
					</div>
				</div>
				<div className="container is-fluid page-content">
					<Alerts />
					<AccountsList moneyMapId={this.state.moneyMap.id} accounts={this.state.moneyMap.accounts} />
				</div>
			</div>
		);
	}
}

const MoneyMap = connect(mapStateToProps, mapDispatchToProps)(ConnectedMoneyMap);
export { MoneyMap };
