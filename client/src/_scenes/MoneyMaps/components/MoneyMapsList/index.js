import React, { Component } from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import MoneyMapTile from './MoneyMapTile';
import Icon from '~/_components/icon';
import './styles.scss';

const mapDispatchToProps = dispatch => {
	return {}
};

const mapStateToProps = state => {
	return {}
}

class ConnectedMoneyMapsList extends Component {
	constructor(props) {
		super(props);
		this.state = {};
	}
	render() {
		let moneyMaps = this.props.moneyMaps;
		return (
			<div className="moneyMapTileList tile is-ancestor">
				{Object.keys(moneyMaps).map((id, index) => 
					<MoneyMapTile key={id} name={moneyMaps[id].name} link={`/money_maps/${id}`}>
						<Icon wrapperClassName="is-large has-text-success" className="fas fa-4x fa-dollar-sign"/>
					</MoneyMapTile>
				)}
				<MoneyMapTile key="add-money-map" name="Add" link="/money_maps/add" classModifiers="add-money-map">
					<Icon wrapperClassName="is-large has-text-white" className="fas fa-4x fa-plus"/>
				</MoneyMapTile>
			</div>
		);
	}
}

const MoneyMapsList = connect(mapStateToProps, mapDispatchToProps)(ConnectedMoneyMapsList);

MoneyMapsList.defaultProps = {
}

MoneyMapsList.propTypes = {
	moneyMaps: PropTypes.object.isRequired
}

export default MoneyMapsList;
