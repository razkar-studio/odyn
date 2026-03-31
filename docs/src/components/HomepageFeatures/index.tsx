import type {ReactNode} from 'react';
import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

type FeatureItem = {
  title: string;
  Svg: React.ComponentType<React.ComponentProps<'svg'>>;
  description: ReactNode;
};

const FeatureList: FeatureItem[] = [
  {
    title: 'Pin Once, Sync Anywhere',
    Svg: require('@site/static/img/undraw_docusaurus_mountain.svg').default,
    description: (
      <>
        Add a dependency with <code>odyn get</code> and its commit hash is
        written to <code>Odyn.lock</code>. Run <code>odyn sync</code> on any
        machine to reproduce the exact same state.
      </>
    ),
  },
  {
    title: 'No Registry, No Surprises',
    Svg: require('@site/static/img/undraw_docusaurus_tree.svg').default,
    description: (
      <>
        Odyn has no central registry and no transitive dependency resolution.
        Every dependency in your project is one you explicitly added.
      </>
    ),
  },
  {
    title: 'Plain Git Under the Hood',
    Svg: require('@site/static/img/undraw_docusaurus_react.svg').default,
    description: (
      <>
        Odyn clones real Git repos into <code>odyn_deps/</code>. No custom
        archive format, no proprietary index. Browse, audit, or replace any
        dependency with standard Git tools.
      </>
    ),
  },
];

function Feature({title, Svg, description}: FeatureItem) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <Svg className={styles.featureSvg} role="img" />
      </div>
      <div className="text--center padding-horiz--md">
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): ReactNode {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
