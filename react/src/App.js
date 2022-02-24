import logo from './wasmedge-horizontal-white.svg';
import React from 'react';
import { Layout, Toast } from '@douyinfe/semi-ui';
import './App.css';

class App extends React.Component {
  constructor(props) {
    super(props);
    try {
      this.state = JSON.parse(props.state);
    }
    catch (e) {
      this.state = { "initialized": false };
    }
  }

  componentDidMount() {
    if (this.state.initialized) {
      Toast.success('SSR Succeed');
    } else {
      Toast.error('SSR Failed');
    }
  }

  render() {
    const { Header, Footer, Content } = Layout;
    let content;
    if (this.state.initialized) {
      content = <><Content>{this.state.msg}</Content></>;
    } else {
      content = <><Content>Failed</Content></>;
    }

    return (
      <>
        <Layout className="major-layout">
          <Header>
            <img src={logo} alt="WasmEdge"></img>
            <h1>SSR Test</h1>
          </Header>
          {content}
          <Footer>2022 LI Rui</Footer>
        </Layout>
      </>
    );
  }
}

export default App;
