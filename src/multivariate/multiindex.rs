use std::ops::{Add,Sub};

#[derive(Clone,PartialEq, Eq,PartialOrd,Ord,Default,Debug)]
pub struct MultiIndex(pub Vec< usize>);
impl MultiIndex {
    pub fn new(multi_index:&Vec<usize>)->Self{
        let mut new_index = multi_index.iter().rev().copied().skip_while(|&x| x==0usize).collect::<Vec<usize>>();
        new_index.reverse();
        if new_index.is_empty(){return MultiIndex(vec![0])}
        MultiIndex(new_index)
    }

    pub fn resize<'a>(&'a mut self,rhs:&'a mut Self)->(&Self,&Self){
        while self.0.len()<rhs.0.len() {
            self.0.append(&mut vec![0usize]);

        }
        while self.0.len()>rhs.0.len() {
            rhs.0.append(&mut vec![0usize]);
        }
        (self,rhs)
    }
    pub fn len(&self)->usize {
        self.0.len()
        
    }
    pub fn is_empty(&self)->bool{
        self.0.is_empty()

    }
    pub fn weight(&self)->usize{
        self.0.iter().sum()
    }
    pub fn zero()->Self {
        MultiIndex::new(&vec![])
    }
    pub fn is_zero(&self)->bool{
        *self==Self::zero()||self.is_empty()
    }
    pub fn is_subtractable_by(&mut self,rhs:&mut Self)->bool{
        if rhs.is_zero(){return false;}
        let (self_resize,rhs_resize)=self.resize(rhs);    
        let matching = self_resize.0.iter().zip(&rhs_resize.0).filter(|&(a, b)| a <= b).count();
        matching==self_resize.len()
    }    
}
impl<'a, 'b> Add<&'b mut MultiIndex> for &'a mut MultiIndex {
    type Output = MultiIndex;
/// # Example
/// 
/// ```
/// use algebra::multivariate::terms::Terms;
/// use algebra::multivariate::multiindex::MultiIndex;
/// use algebra::multivariate::multivariatepoly::MultivariatePoly;
/// let mut multi_index_1 = MultiIndex::new(&vec![2,0]);
/// let mut multi_index_2=MultiIndex::new(&vec![1,0,1]);
/// let sum = &mut multi_index_1+&mut multi_index_2;
/// let expected_sum = MultiIndex::new(&vec![3,0,1]);
/// assert_eq!(sum,expected_sum);
///  
/// ```
    fn add(self:&'a mut MultiIndex, rhs: &'b mut MultiIndex) -> Self::Output {
        if self.is_zero() {return rhs.clone();}
        if rhs.is_zero() {return self.clone();}
        let (self_resize,rhs_resize)=self.resize(rhs);
        let mut sum=Vec::new();
        for (a, b) in self_resize.0.iter().zip(rhs_resize.0.iter()) {
            sum.push(a + b);
        }
        MultiIndex::new(&sum)
        
    }
}
impl<'a, 'b> Sub<&'b mut MultiIndex> for &'a mut MultiIndex {
    type Output = MultiIndex;
/// # Example
/// 
/// ```
/// use algebra::multivariate::multiindex::MultiIndex;
/// use algebra::multivariate::terms::Terms;
/// use algebra::multivariate::multivariatepoly::MultivariatePoly;
/// let mut multi_index_1 = MultiIndex::new(&vec![2,2,3,0]);
/// let mut multi_index_2=MultiIndex::new(&vec![1,0,1]);
/// let sub = &mut multi_index_1-&mut multi_index_2;
/// let expected_sub = MultiIndex::new(&vec![1,2,2,0]);
/// assert_eq!(sub,expected_sub);
///  
/// ```
    fn sub(self:&'a mut MultiIndex, rhs: &'b mut MultiIndex) -> Self::Output {
        if !rhs.is_subtractable_by(self) {panic!("Cant subtract {:?} from {:?}",self,rhs)}
        let (self_resize,rhs_resize)=self.resize(rhs);
        let mut sub=Vec::new();
        for (a, b) in self_resize.0.iter().zip(rhs_resize.0.iter()) {
            sub.push(a - b);
        }
        MultiIndex::new(&sub)
        
    }
}

