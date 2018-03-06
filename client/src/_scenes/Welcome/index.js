import React, { Component } from 'react';
import { Link } from 'react-router-dom';
import { connect } from 'react-redux'
import './styles.scss';

const mapStateToProps = state => ({
});

class ConnectedWelcome extends Component {
	constructor(props) {
		super(props);
		this.state = {};
	}
	render() {
		return (
			<div>
				<section className="hero is-primary is-medium">
					<div className="hero-body">
						<div className="container">
							<h1 className="title">
								Money Map
							</h1>
							<h2 className="subtitle">
								a finance management solution
							</h2>
						</div>
					</div>
				</section>
				<section className="section">
					<div className="container">
						<h2 className="title is-2">Welcome.</h2>
						<p>Learn all about Money Map here.</p>
						Ready to <Link to={'/login'} >Login</Link>?
					</div>
				</section>
			</div>
		);
	}
}

const Welcome = connect(mapStateToProps)(ConnectedWelcome);
export default Welcome;
