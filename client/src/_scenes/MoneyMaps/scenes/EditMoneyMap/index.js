import React, { Component } from 'react';
import { connect } from 'react-redux';
import Alerts from '~/_components/alerts';
import EditMoneyMapForm from './components/EditMoneyMapForm';
import './styles.scss';

const mapDispatchToProps = dispatch => {
	return {
	};
};

const mapStateToProps = state => {
	return {}
}

class ConnectedEditMoneyMap extends Component {
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
								Edit Money Map
							</h1>
						</div>
					</div>
				</div>
				<div className="container is-fluid page-content">
					<Alerts />
					<EditMoneyMapForm moneyMapId={this.props.match.params.id} />
				</div>
			</div>
		);
	}
}

const EditMoneyMap = connect(mapStateToProps, mapDispatchToProps)(ConnectedEditMoneyMap);
export { EditMoneyMap };
